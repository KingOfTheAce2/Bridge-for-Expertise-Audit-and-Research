import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import '../styles/Sidebar.css';

interface NavItem {
  to: string;
  icon: string;
  label: string;
}

const mainNavItems: NavItem[] = [
  { to: '/', icon: '🏠', label: 'Home' },
  { to: '/chat', icon: '💬', label: 'AI Chat' },
];

const toolsNavItems: NavItem[] = [
  { to: '/models', icon: '🤖', label: 'AI Models' },
  { to: '/ner-models', icon: '🧠', label: 'NER Models' },
  { to: '/pii-protection', icon: '🛡️', label: 'PII Protection' },
];

const infoNavItems: NavItem[] = [
  { to: '/about-ai', icon: 'ℹ️', label: 'About AI' },
  { to: '/settings', icon: '⚙️', label: 'Settings' },
  { to: '/about', icon: '📋', label: 'About' },
];

const NavLink: React.FC<{ item: NavItem }> = ({ item }) => {
  const { pathname } = useLocation();
  const isActive = pathname === item.to;

  return (
    <Link
      to={item.to}
      className={`nav-link ${isActive ? 'active' : ''}`}
    >
      <span className="nav-link-icon">{item.icon}</span>
      {item.label}
    </Link>
  );
};

const Sidebar: React.FC = () => {
  const toggleTheme = () => {
    const root = document.documentElement;
    root.classList.toggle('dark');
  };

  return (
    <aside className="sidebar">
      {/* Header */}
      <div className="sidebar-header">
        <div className="sidebar-logo">
          <div className="sidebar-logo-icon">
            <span role="img" aria-label="Bear">🐻</span>
          </div>
          <div className="sidebar-brand">
            <h1 className="sidebar-brand-name">BEAR LLM AI</h1>
            <p className="sidebar-brand-version">v0.0.20</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="sidebar-nav">
        {/* Main */}
        <div className="nav-section">
          <div className="nav-section-title">Main</div>
          {mainNavItems.map((item) => (
            <NavLink key={item.to} item={item} />
          ))}
        </div>

        {/* Tools */}
        <div className="nav-section">
          <div className="nav-section-title">Tools</div>
          {toolsNavItems.map((item) => (
            <NavLink key={item.to} item={item} />
          ))}
        </div>

        {/* Info */}
        <div className="nav-section">
          <div className="nav-section-title">Info</div>
          {infoNavItems.map((item) => (
            <NavLink key={item.to} item={item} />
          ))}
        </div>
      </nav>

      {/* Footer */}
      <div className="sidebar-footer">
        <button
          className="theme-toggle-btn"
          onClick={toggleTheme}
          aria-label="Toggle theme"
        >
          <span role="img" aria-label="Theme">🌓</span>
          Toggle Theme
        </button>
      </div>
    </aside>
  );
};

export default Sidebar;
