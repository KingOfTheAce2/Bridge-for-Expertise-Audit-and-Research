import React from "react";

const CaseDetail: React.FC<{ caseData: any }> = ({ caseData }) => {
  return (
    <div className="flex-1 p-6">
      <h2 className="text-2xl font-semibold mb-4">{caseData.name}</h2>
      <div className="space-y-2">
        <p><strong>Client:</strong> {caseData.clientName}</p>
        <p><strong>Case Number:</strong> {caseData.caseNumber}</p>
        <p><strong>Status:</strong> {caseData.status ?? "active"}</p>
        <p className="mt-4 text-gray-700 dark:text-gray-300">
          {caseData.description ?? "No description available."}
        </p>
      </div>
    </div>
  );
};

export default CaseDetail;
