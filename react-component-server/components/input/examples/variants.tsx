import React from 'react';
import { Input } from '../component';

/**
 * Input Appearance and Size Variants
 * Demonstrates all available input appearances and sizes
 */
export const InputVariantExamples: React.FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '20px', maxWidth: '400px' }}>
      <h3>Input Appearances</h3>
      
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <Input
          appearance="outline"
          placeholder="Outline appearance (default)"
        />
        
        <Input
          appearance="underline"
          placeholder="Underline appearance"
        />
        
        <Input
          appearance="filled-darker"
          placeholder="Filled darker appearance"
        />
        
        <Input
          appearance="filled-lighter"
          placeholder="Filled lighter appearance"
        />
      </div>
      
      <h3>Input Sizes</h3>
      
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <Input
          size="small"
          placeholder="Small size"
          appearance="outline"
        />
        
        <Input
          size="medium"
          placeholder="Medium size (default)"
          appearance="outline"
        />
        
        <Input
          size="large"
          placeholder="Large size"
          appearance="outline"
        />
      </div>
      
      <h3>Combined Variants</h3>
      
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <Input
          size="large"
          appearance="filled-darker"
          placeholder="Large filled input"
        />
        
        <Input
          size="small"
          appearance="underline"
          placeholder="Small underline input"
        />
      </div>
    </div>
  );
};

export default InputVariantExamples;