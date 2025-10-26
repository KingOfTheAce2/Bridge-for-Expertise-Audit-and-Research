import React, { useState } from "react";
import CaseList from "../components/CaseList";
import CaseDetail from "../components/CaseDetail";

const CasesPage: React.FC = () => {
  const [cases, setCases] = useState<any[]>([]);
  const [selectedCase, setSelectedCase] = useState<any | null>(null);

  return (
    <div className="flex h-full">
      <CaseList cases={cases} onSelect={setSelectedCase} />
      {selectedCase && <CaseDetail caseData={selectedCase} />}
    </div>
  );
};

export default CasesPage;
