// src/components/AIBadge.tsx
interface AIBadgeProps {
  model?: string;
  timestamp?: string;
  wasEdited?: boolean;
  className?: string;
}

export const AIBadge: React.FC<AIBadgeProps> = ({
  model,
  timestamp,
  wasEdited,
  className,
}) => {
  return (
    <div className={`inline-flex items-center gap-2 px-3 py-1 rounded-full bg-blue-100 text-blue-800 text-sm ${className}`}>
      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
        <path d="M10 2a8 8 0 100 16 8 8 0 000-16zm1 11H9v-2h2v2zm0-4H9V5h2v4z"/>
      </svg>

      <span className="font-medium">
        AI Generated
        {wasEdited && " (Edited)"}
      </span>

      {model && (
        <span className="text-xs opacity-75">
          {model}
        </span>
      )}

      {timestamp && (
        <span className="text-xs opacity-75">
          {new Date(timestamp).toLocaleString()}
        </span>
      )}
    </div>
  );
};

// Different badge variants
export const AIAssistedBadge = () => (
  <span className="px-2 py-1 rounded bg-purple-100 text-purple-800 text-xs">
    AI-Assisted
  </span>
);

export const HumanContentBadge = () => (
  <span className="px-2 py-1 rounded bg-gray-100 text-gray-800 text-xs">
    Human Written
  </span>
);

// Message component with AI badge
interface MessageProps {
  role: 'user' | 'assistant';
  content: string;
  isAiGenerated?: boolean;
  wasEdited?: boolean;
}

const Message: React.FC<MessageProps> = ({ role, content, isAiGenerated, wasEdited }) => {
  return (
    <div className={`message ${role === 'user' ? 'user' : 'assistant'}`}>
      {isAiGenerated && <AIBadge wasEdited={wasEdited} />}
      <div className="content">{content}</div>
    </div>
  );
};
