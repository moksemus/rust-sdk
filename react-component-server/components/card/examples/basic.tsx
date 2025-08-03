import React from 'react';
import { Card } from '../component';

/**
 * Basic Card Examples
 * Demonstrates simple card usage with different appearances and content
 */
export const BasicCardExamples: React.FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '20px', maxWidth: '400px' }}>
      <h3>Card Appearances</h3>
      
      <Card appearance="filled">
        <div style={{ padding: '16px' }}>
          <h4>Filled Card (Default)</h4>
          <p>This is a filled card with some content inside.</p>
        </div>
      </Card>
      
      <Card appearance="outline">
        <div style={{ padding: '16px' }}>
          <h4>Outline Card</h4>
          <p>This card has an outline appearance.</p>
        </div>
      </Card>
      
      <Card appearance="subtle">
        <div style={{ padding: '16px' }}>
          <h4>Subtle Card</h4>
          <p>This card has a subtle appearance.</p>
        </div>
      </Card>
      
      <Card appearance="filled-alternative">
        <div style={{ padding: '16px' }}>
          <h4>Filled Alternative Card</h4>
          <p>This card uses the alternative filled appearance.</p>
        </div>
      </Card>

      <h3>Card Sizes</h3>
      
      <Card size="small" appearance="outline">
        <div style={{ padding: '12px' }}>
          <h5>Small Card</h5>
          <p>Compact content.</p>
        </div>
      </Card>
      
      <Card size="medium" appearance="outline">
        <div style={{ padding: '16px' }}>
          <h4>Medium Card (Default)</h4>
          <p>Standard sized content.</p>
        </div>
      </Card>
      
      <Card size="large" appearance="outline">
        <div style={{ padding: '20px' }}>
          <h3>Large Card</h3>
          <p>More spacious content area with larger padding.</p>
        </div>
      </Card>
    </div>
  );
};

export default BasicCardExamples;