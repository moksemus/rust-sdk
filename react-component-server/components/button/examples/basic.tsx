import React from 'react';
import { Button } from '../component';

/**
 * Basic Button Examples
 * Demonstrates simple button usage with different content types
 */
export const BasicButtonExamples: React.FC = () => {
  return (
    <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
      {/* Simple text button */}
      <Button>
        Click me
      </Button>
      
      {/* Button with emoji */}
      <Button>
        🚀 Launch
      </Button>
      
      {/* Button with longer text */}
      <Button>
        Save Document
      </Button>
      
      {/* Button with click handler */}
      <Button onClick={() => alert('Hello from Button!')}>
        Show Alert
      </Button>
    </div>
  );
};

export default BasicButtonExamples;