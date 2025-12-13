import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import AIContentBadge from '../components/AIContentBadge';
import '../styles/Chat.css';

interface ChatMessage {
  role: string;
  content: string;
}

interface SystemPrompt {
  id: string;
  name: string;
  prompt: string;
}

const Chat: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [modelStatus, setModelStatus] = useState<string>('not_loaded');
  const [systemPrompts, setSystemPrompts] = useState<SystemPrompt[]>([]);
  const [selectedPrompt, setSelectedPrompt] = useState<string>('assistant');
  const [streamingMessage, setStreamingMessage] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadSystemPrompts();
    checkModelStatus();

    // Listen for streaming tokens
    const unlisten = listen<any>('ai-token', (event) => {
      const { token, is_final } = event.payload;

      if (is_final) {
        // Finalize the message
        setMessages((prev) => [
          ...prev,
          { role: 'assistant', content: streamingMessage + ' ' + token },
        ]);
        setStreamingMessage('');
        setIsGenerating(false);
      } else {
        // Append token
        setStreamingMessage((prev) => prev + ' ' + token);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [streamingMessage]);

  useEffect(() => {
    scrollToBottom();
  }, [messages, streamingMessage]);

  const loadSystemPrompts = async () => {
    try {
      const prompts = await invoke<SystemPrompt[]>('get_system_prompts');
      setSystemPrompts(prompts);
    } catch (error) {
      console.error('Failed to load system prompts:', error);
    }
  };

  const checkModelStatus = async () => {
    try {
      const status = await invoke<string>('get_ai_model_status');
      setModelStatus(status);
    } catch (error) {
      console.error('Failed to check model status:', error);
    }
  };

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleSendMessage = async () => {
    if (!inputMessage.trim() || isGenerating) return;

    if (modelStatus !== 'loaded') {
      alert('Please load an AI model first from the Models page.');
      return;
    }

    const userMessage: ChatMessage = {
      role: 'user',
      content: inputMessage.trim(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInputMessage('');
    setIsGenerating(true);

    try {
      const selectedPromptObj = systemPrompts.find((p) => p.id === selectedPrompt);

      await invoke('generate_ai_response_stream', {
        request: {
          conversation_id: null,
          messages: [...messages, userMessage],
          system_prompt: selectedPromptObj?.prompt || null,
          temperature: 0.7,
          max_tokens: 2048,
        },
      });
    } catch (error) {
      console.error('Failed to generate response:', error);
      setMessages((prev) => [
        ...prev,
        {
          role: 'assistant',
          content: `Error: ${error}`,
        },
      ]);
      setIsGenerating(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const clearConversation = () => {
    if (confirm('Are you sure you want to clear the conversation?')) {
      setMessages([]);
      setStreamingMessage('');
    }
  };

  const getStatusColor = () => {
    switch (modelStatus) {
      case 'loaded':
        return '#48bb78';
      case 'loading':
        return '#ed8936';
      case 'error':
        return '#f56565';
      default:
        return '#cbd5e0';
    }
  };

  const getStatusText = () => {
    switch (modelStatus) {
      case 'loaded':
        return 'Model Loaded';
      case 'loading':
        return 'Loading Model...';
      case 'error':
        return 'Model Error';
      default:
        return 'No Model Loaded';
    }
  };

  return (
    <div className="chat-page">
      {/* Header */}
      <div className="chat-header">
        <div className="header-left">
          <h1>AI Chat</h1>
          <div className="model-status" style={{ borderColor: getStatusColor() }}>
            <div
              className="status-indicator"
              style={{ backgroundColor: getStatusColor() }}
            />
            <span>{getStatusText()}</span>
          </div>
        </div>

        <div className="header-controls">
          <select
            value={selectedPrompt}
            onChange={(e) => setSelectedPrompt(e.target.value)}
            className="prompt-selector"
          >
            {systemPrompts.map((prompt) => (
              <option key={prompt.id} value={prompt.id}>
                {prompt.name}
              </option>
            ))}
          </select>

          <button onClick={clearConversation} className="btn btn-secondary">
            Clear Chat
          </button>
        </div>
      </div>

      {/* Messages Area */}
      <div className="messages-container">
        {messages.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">💬</div>
            <h2>Start a conversation</h2>
            <p>
              Ask questions, get help with drafting, or explore ideas. Your AI assistant
              is ready to help.
            </p>
            {modelStatus !== 'loaded' && (
              <div className="warning-box">
                <strong>⚠️ No AI model loaded</strong>
                <p>Please visit the Models page to download and load an AI model.</p>
              </div>
            )}
          </div>
        )}

        {messages.map((message, index) => (
          <div key={index} className={`message message-${message.role}`}>
            <div className="message-header">
              <span className="message-role">
                {message.role === 'user' ? '👤 You' : '🤖 AI Assistant'}
              </span>
              {message.role === 'assistant' && (
                <AIContentBadge source="ai" size="small" showIcon={false} />
              )}
            </div>
            <div className="message-content">{message.content}</div>
          </div>
        ))}

        {/* Streaming message */}
        {isGenerating && streamingMessage && (
          <div className="message message-assistant streaming">
            <div className="message-header">
              <span className="message-role">🤖 AI Assistant</span>
              <AIContentBadge source="ai" size="small" showIcon={false} />
            </div>
            <div className="message-content">
              {streamingMessage}
              <span className="cursor-blink">▋</span>
            </div>
          </div>
        )}

        {/* Loading indicator */}
        {isGenerating && !streamingMessage && (
          <div className="message message-assistant">
            <div className="message-header">
              <span className="message-role">🤖 AI Assistant</span>
            </div>
            <div className="message-content">
              <div className="typing-indicator">
                <span></span>
                <span></span>
                <span></span>
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div className="input-container">
        <div className="input-wrapper">
          <textarea
            value={inputMessage}
            onChange={(e) => setInputMessage(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Type your message... (Shift+Enter for new line)"
            className="message-input"
            rows={3}
            disabled={isGenerating}
          />
          <button
            onClick={handleSendMessage}
            disabled={!inputMessage.trim() || isGenerating || modelStatus !== 'loaded'}
            className="send-button"
          >
            {isGenerating ? '⏳ Generating...' : '🚀 Send'}
          </button>
        </div>

        <div className="input-footer">
          <span className="hint">
            💡 Press Enter to send, Shift+Enter for new line
          </span>
          {modelStatus !== 'loaded' && (
            <span className="warning">⚠️ Load a model to start chatting</span>
          )}
        </div>
      </div>
    </div>
  );
};

export default Chat;
