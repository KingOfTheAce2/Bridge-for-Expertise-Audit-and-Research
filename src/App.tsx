import React from 'react'
import { Routes, Route } from 'react-router-dom'
import Sidebar from './components/Sidebar'
import Footer from './components/Footer'
import HomePage from './pages/Home'
import ModelsPage from './pages/Models'
import PIIProtectionPage from './pages/PIIProtection'
import SettingsPage from './pages/Settings'
import AboutPage from './pages/About'

export default function App() {
  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <main className="flex-1 overflow-auto">
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/models" element={<ModelsPage />} />
            <Route path="/pii-protection" element={<PIIProtectionPage />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="/about" element={<AboutPage />} />
          </Routes>
        </main>
        <Footer />
      </div>
    </div>
  )
}
