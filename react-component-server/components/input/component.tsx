import React from 'react';
import { Input as FluentInput } from '@fluentui/react-components';
import { InputProps } from './types';

/**
 * Input component based on Fluent UI Input
 * 
 * A text input field that allows users to enter and edit text with various 
 * appearances and validation states. Supports different input types and sizes.
 */
export const Input: React.FC<InputProps> = ({
  value,
  placeholder,
  type = 'text',
  size = 'medium',
  appearance = 'outline',
  disabled = false,
  onChange,
  ...rest
}) => {
  return (
    <FluentInput
      value={value}
      placeholder={placeholder}
      type={type}
      size={size}
      appearance={appearance}
      disabled={disabled}
      onChange={onChange}
      {...rest}
    />
  );
};

export default Input;