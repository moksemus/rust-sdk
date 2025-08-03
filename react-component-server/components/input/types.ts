import { InputProps as FluentInputProps } from '@fluentui/react-components';

/**
 * Props interface for the Input component
 * Extends Fluent UI InputProps with additional customizations
 */
export interface InputProps extends Omit<FluentInputProps, 'as'> {
  /** The controlled value of the input */
  value?: string;
  
  /** Placeholder text when input is empty */
  placeholder?: string;
  
  /** The type of input field */
  type?: 'text' | 'email' | 'password' | 'tel' | 'url';
  
  /** The size of the input field */
  size?: 'small' | 'medium' | 'large';
  
  /** The visual appearance of the input */
  appearance?: 'outline' | 'underline' | 'filled-darker' | 'filled-lighter';
  
  /** Whether the input is disabled */
  disabled?: boolean;
  
  /** Function called when the input value changes */
  onChange?: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

export default InputProps;