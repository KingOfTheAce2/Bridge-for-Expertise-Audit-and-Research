import React from 'react'
import { Link, useLocation } from 'react-router-dom'

const NavLink = ({ to, children }: { to: string; children: React.ReactNode }) => {
  const { pathname } = useLocation()
  const active = pathname === to
  return (
    <Link
      to={to}
      className={
        'block px-4 py-2 rounded-md transition-colors ' +
        (active
          ? 'bg-gray-200 dark:bg-gray-700 font-medium'
          : 'hover:bg-gray-100 dark:hover:bg-gray-800')
      }
    >
      {children}
    </Link>
  )
}

const Sidebar: React.FC = () => {
  return (
    <aside className="w-64 bg-white dark:bg-gray-850 border-r border-gray-200 dark:border-gray-800 flex flex-col">
      <div className="p-4">
        <h1 className="text-xl font-bold">BEAR LLM AI</h1>
        <p className="text-sm text-gray-500">v0.0.20</p>
      </div>
      <nav className="px-2 space-y-1">
        <NavLink to="/">Home</NavLink>
        <NavLink to="/chat">AI Chat</NavLink>
        <NavLink to="/models">AI Models</NavLink>
        <NavLink to="/ner-models">NER Models</NavLink>
        <NavLink to="/pii-protection">PII Protection</NavLink>
        <NavLink to="/about-ai">About AI</NavLink>
        <NavLink to="/settings">Settings</NavLink>
        <NavLink to="/about">About</NavLink>
      </nav>
      <div className="mt-auto p-3 text-xs text-gray-500">
        <button
          className="w-full rounded-md px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-800"
          onClick={() => {
            const root = document.documentElement
            root.classList.toggle('dark')
          }}
          aria-label="Toggle theme"
        >
          Toggle Theme
        </button>
      </div>
    </aside>
  )
}

export default Sidebar
