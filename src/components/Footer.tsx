import React from 'react'

export default function Footer() {
  return (
    <footer className="bg-gray-100 dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 py-4 px-6">
      <div className="flex flex-col sm:flex-row justify-between items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
        <div className="flex flex-col sm:flex-row items-center gap-2">
          <span className="font-medium">Authors:</span>
          <span>Burrough / Machon / Oranje / Frakes / Visser</span>
        </div>
        <div className="flex flex-col sm:flex-row items-center gap-2">
          <span className="font-medium">License:</span>
          <a
            href="https://creativecommons.org/licenses/by/4.0/"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 dark:text-blue-400 hover:underline"
          >
            Creative Commons Attribution 4.0 International (CC BY 4.0)
          </a>
        </div>
      </div>
    </footer>
  )
}
