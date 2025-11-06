import { invoke } from '@tauri-apps/api/core';

export interface Entity {
  entity_type: {
    Person?: null;
    Organization?: null;
    Location?: null;
    Date?: null;
    Money?: null;
    Law?: null;
    Case?: null;
    Email?: null;
    Phone?: null;
    Identification?: null;
    TechnicalIdentifier?: null;
  };
  text: string;
  start: number;
  end: number;
  confidence: number;
  replacement?: string;
}

export interface AnonymizationSettings {
  entity_types: string[];
  confidence_threshold: number;
  preserve_legal_references: boolean;
  consistent_replacement: boolean;
  language: string;
}

export interface AnonymizationResult {
  original_text: string;
  anonymized_text: string;
  entities: Entity[];
  replacements: Array<[string, string]>;
}

export interface EntityStatistics {
  entity_counts: Array<[string, number]>;
  total_entities: number;
}

class PIIService {
  /**
   * Anonymize text
   */
  async anonymizeText(
    text: string,
    settings?: AnonymizationSettings
  ): Promise<AnonymizationResult> {
    try {
      const result = await invoke<AnonymizationResult>('anonymize_text', {
        request: {
          text,
          settings,
        },
      });
      return result;
    } catch (error) {
      console.error('Failed to anonymize text:', error);
      throw error;
    }
  }

  /**
   * Anonymize multiple texts while maintaining consistency
   */
  async anonymizeBatch(
    texts: string[],
    settings?: AnonymizationSettings
  ): Promise<AnonymizationResult[]> {
    try {
      const results = await invoke<AnonymizationResult[]>('anonymize_batch', {
        request: {
          texts,
          settings,
        },
      });
      return results;
    } catch (error) {
      console.error('Failed to anonymize batch:', error);
      throw error;
    }
  }

  /**
   * Clear PII replacement mappings (start fresh)
   */
  async clearReplacements(): Promise<string> {
    try {
      const result = await invoke<string>('clear_pii_replacements');
      return result;
    } catch (error) {
      console.error('Failed to clear replacements:', error);
      throw error;
    }
  }

  /**
   * Get statistics about detected entities
   */
  async getStatistics(): Promise<EntityStatistics> {
    try {
      const stats = await invoke<EntityStatistics>('get_pii_statistics');
      return stats;
    } catch (error) {
      console.error('Failed to get statistics:', error);
      throw error;
    }
  }

  /**
   * Get default anonymization settings
   */
  async getDefaultSettings(): Promise<AnonymizationSettings> {
    try {
      const settings = await invoke<AnonymizationSettings>('get_default_pii_settings');
      return settings;
    } catch (error) {
      console.error('Failed to get default settings:', error);
      throw error;
    }
  }

  /**
   * Get available entity types
   */
  async getEntityTypes(): Promise<string[]> {
    try {
      const types = await invoke<string[]>('get_entity_types');
      return types;
    } catch (error) {
      console.error('Failed to get entity types:', error);
      throw error;
    }
  }

  /**
   * Detect entities without anonymizing
   */
  async detectEntities(text: string): Promise<Entity[]> {
    try {
      const entities = await invoke<Entity[]>('detect_pii_entities', { text });
      return entities;
    } catch (error) {
      console.error('Failed to detect entities:', error);
      throw error;
    }
  }

  /**
   * Format entity type for display
   */
  formatEntityType(entityType: string): string {
    const displayNames: Record<string, string> = {
      PERSON: 'Person',
      ORGANIZATION: 'Organization',
      LOCATION: 'Location',
      DATE: 'Date',
      MONEY: 'Money',
      LAW: 'Legal Reference',
      CASE: 'Case Number',
      EMAIL: 'Email',
      PHONE: 'Phone',
      IDENTIFICATION: 'ID Number',
      TECHNICAL_IDENTIFIER: 'Technical ID',
    };
    return displayNames[entityType] || entityType;
  }

  /**
   * Get color for entity type
   */
  getEntityColor(entityType: string): string {
    const colors: Record<string, string> = {
      PERSON: '#e53e3e',
      ORGANIZATION: '#dd6b20',
      LOCATION: '#d69e2e',
      DATE: '#38a169',
      MONEY: '#319795',
      LAW: '#3182ce',
      CASE: '#805ad5',
      EMAIL: '#d53f8c',
      PHONE: '#00b5d8',
      IDENTIFICATION: '#e53e3e',
      TECHNICAL_IDENTIFIER: '#718096',
    };
    return colors[entityType] || '#718096';
  }
}

export const piiService = new PIIService();
