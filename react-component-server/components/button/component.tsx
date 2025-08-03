import React from 'react';
import { Button as FluentButton } from '@fluentui/react-components';
import { ButtonProps } from './types';

/**
 * Button component based on Fluent UI Button
 * 
 * A versatile button component that supports various appearances, sizes, and states.
 * Perfect for triggering actions, submitting forms, or navigating within applications.
 */
export const Button: React.FC<ButtonProps> = ({
  children,
  appearance = 'secondary',
  shape = 'rounded',
  size = 'medium',
  disabled = false,
  onClick,
  ...rest
}) => {
  return (
    <FluentButton
      appearance={appearance}
      shape={shape}
      size={size}
      disabled={disabled}
      onClick={onClick}
      {...rest}
    >
      {children}
    </FluentButton>
  );
};

export default Button;