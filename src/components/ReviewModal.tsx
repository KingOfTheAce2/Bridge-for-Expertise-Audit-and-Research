// src/components/ReviewModal.tsx
import React, { useState } from 'react';
import { AIBadge } from './AIBadge';

interface ReviewModalProps {
  title: string;
  content: string;
  metadata?: {
    source?: 'ai' | 'user';
    model?: string;
    timestamp?: string;
  };
  onApprove: (edited: string) => void;
  onReject: () => void;
  onEdit: (edited: string) => void;
}

const ReviewModal: React.FC<ReviewModalProps> = ({
  title,
  content,
  metadata,
  onApprove,
  onReject,
  onEdit,
}) => {
  const [editedContent, setEditedContent] = useState(content);
  const [isEditing, setIsEditing] = useState(false);

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="max-w-4xl p-6 bg-white rounded-lg shadow-xl">
        <h2 className="text-xl font-bold mb-4">{title}</h2>

        {/* Show AI badge if applicable */}
        {metadata?.source === 'ai' && (
          <AIBadge model={metadata.model} />
        )}

        {/* Preview mode */}
        {!isEditing ? (
          <div className="prose max-w-none mb-6 p-4 bg-gray-50 rounded">
            {editedContent}
          </div>
        ) : (
          <textarea
            className="w-full h-64 p-4 border rounded mb-6"
            value={editedContent}
            onChange={(e) => setEditedContent(e.target.value)}
          />
        )}

        {/* Action buttons */}
        <div className="flex gap-3 justify-end">
          <button
            className="btn-secondary"
            onClick={onReject}
          >
            Reject
          </button>

          {!isEditing ? (
            <>
              <button
                className="btn-secondary"
                onClick={() => setIsEditing(true)}
              >
                Edit
              </button>
              <button
                className="btn-primary"
                onClick={() => onApprove(editedContent)}
              >
                Approve
              </button>
            </>
          ) : (
            <>
              <button
                className="btn-secondary"
                onClick={() => {
                  setEditedContent(content);
                  setIsEditing(false);
                }}
              >
                Cancel Edit
              </button>
              <button
                className="btn-primary"
                onClick={() => {
                  onEdit(editedContent);
                  setIsEditing(false);
                }}
              >
                Save Changes
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
};
