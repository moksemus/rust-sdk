import React from 'react';
import { Button } from '../component';

/**
 * Button Size Examples
 * Demonstrates all available button sizes
 */
export const ButtonSizeExamples: React.FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <h3>Button Sizes</h3>
      
      <div style={{ display: 'flex', gap: '12px', alignItems: 'center', flexWrap: 'wrap' }}>
        <Button size="small" appearance="primary">
          Small Button
        </Button>
        
        <Button size="medium" appearance="primary">
          Medium Button (Default)
        </Button>
        
        <Button size="large" appearance="primary">
          Large Button
        </Button>
      </div>
      
      <h3>Size with Different Appearances</h3>
      
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <Button size="small" appearance="outline">Small Outline</Button>
          <Button size="medium" appearance="outline">Medium Outline</Button>
          <Button size="large" appearance="outline">Large Outline</Button>
        </div>
        
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <Button size="small" appearance="subtle">Small Subtle</Button>
          <Button size="medium" appearance="subtle">Medium Subtle</Button>
          <Button size="large" appearance="subtle">Large Subtle</Button>
        </div>
      </div>
    </div>
  );
};

export default ButtonSizeExamples;