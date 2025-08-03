import React from 'react';
import { Button } from '../component';

/**
 * Button Appearance Variants
 * Demonstrates all available button appearance styles
 */
export const ButtonVariantExamples: React.FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <h3>Button Appearances</h3>
      
      <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
        <Button appearance="primary">
          Primary
        </Button>
        
        <Button appearance="outline">
          Outline
        </Button>
        
        <Button appearance="subtle">
          Subtle
        </Button>
        
        <Button appearance="secondary">
          Secondary (Default)
        </Button>
        
        <Button appearance="transparent">
          Transparent
        </Button>
      </div>
      
      <h3>Button Shapes</h3>
      
      <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
        <Button shape="rounded" appearance="primary">
          Rounded
        </Button>
        
        <Button shape="circular" appearance="primary">
          ⭐
        </Button>
        
        <Button shape="square" appearance="primary">
          ■
        </Button>
      </div>
      
      <h3>Disabled States</h3>
      
      <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
        <Button appearance="primary" disabled>
          Disabled Primary
        </Button>
        
        <Button appearance="outline" disabled>
          Disabled Outline
        </Button>
      </div>
    </div>
  );
};

export default ButtonVariantExamples;