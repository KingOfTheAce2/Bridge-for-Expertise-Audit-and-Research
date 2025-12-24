import React from 'react'
import { Routes, Route } from 'react-router-dom'
import Sidebar from './components/Sidebar'
import HomePage from './pages/Home'
import ChatPage from './pages/Chat'
import ModelsPage from './pages/Models'
import NERModelsPage from './pages/NERModels'
import PIIProtectionPage from './pages/PIIProtection'
import AboutAIPage from './pages/AboutAI'
import SettingsPage from './pages/Settings'
import AboutPage from './pages/About'

export default function App() {
  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <Sidebar />
      <main className="flex-1 overflow-auto">
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/chat" element={<ChatPage />} />
          <Route path="/models" element={<ModelsPage />} />
          <Route path="/ner-models" element={<NERModelsPage />} />
          <Route path="/pii-protection" element={<PIIProtectionPage />} />
          <Route path="/about-ai" element={<AboutAIPage />} />
          <Route path="/settings" element={<SettingsPage />} />
          <Route path="/about" element={<AboutPage />} />
        </Routes>
      </main>
    </div>
  )
}
