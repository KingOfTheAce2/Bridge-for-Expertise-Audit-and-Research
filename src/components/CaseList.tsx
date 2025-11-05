import React from "react";

interface Case {
  id: number;
  name: string;
  clientName: string;
  caseNumber: string;
}

interface Props {
  cases: Case[];
  onSelect: (caseItem: Case) => void;
}

const CaseList: React.FC<Props> = ({ cases, onSelect }) => {
  return (
    <div className="w-80 border-r border-gray-200 dark:border-gray-700">
      <div className="p-4">
        <button className="btn-primary w-full">New Case</button>
      </div>
      <div className="divide-y divide-gray-100 dark:divide-gray-800">
        {cases.length === 0 ? (
          <div className="p-8 text-center text-gray-500 dark:text-gray-400">
            <p className="mb-2">No cases found</p>
            <p className="text-sm">Click "New Case" to get started</p>
          </div>
        ) : (
          cases.map((c) => (
            <div
              key={c.id}
              className="p-4 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
              onClick={() => onSelect(c)}
            >
              <h3 className="font-medium text-gray-900 dark:text-gray-100">
                {c.name}
              </h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {c.clientName}
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-500">
                {c.caseNumber}
              </p>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default CaseList;
