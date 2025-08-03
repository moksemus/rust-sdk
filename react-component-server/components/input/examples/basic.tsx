import React, { useState } from 'react';
import { Input } from '../component';

/**
 * Basic Input Examples
 * Demonstrates simple input usage with different types and states
 */
export const BasicInputExamples: React.FC = () => {
  const [textValue, setTextValue] = useState('');
  const [emailValue, setEmailValue] = useState('');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', maxWidth: '400px' }}>
      <h3>Basic Input Examples</h3>
      
      {/* Basic text input */}
      <Input
        placeholder="Enter your name"
        value={textValue}
        onChange={(e) => setTextValue(e.target.value)}
      />
      
      {/* Email input */}
      <Input
        type="email"
        placeholder="Enter your email"
        value={emailValue}
        onChange={(e) => setEmailValue(e.target.value)}
      />
      
      {/* Password input */}
      <Input
        type="password"
        placeholder="Enter your password"
      />
      
      {/* Disabled input */}
      <Input
        placeholder="This input is disabled"
        disabled
      />
      
      {/* Read-only input with value */}
      <Input
        value="This is read-only"
        readOnly
      />
    </div>
  );
};

export default BasicInputExamples;