import { ButtonProps as FluentButtonProps } from '@fluentui/react-components';

/**
 * Props interface for the Button component
 * Extends Fluent UI ButtonProps with additional customizations
 */
export interface ButtonProps extends Omit<FluentButtonProps, 'as'> {
  /** The content to display inside the button */
  children?: React.ReactNode;
  
  /** The visual appearance style of the button */
  appearance?: 'primary' | 'outline' | 'subtle' | 'secondary' | 'transparent';
  
  /** The shape styling of the button */
  shape?: 'rounded' | 'circular' | 'square';
  
  /** The size of the button */
  size?: 'small' | 'medium' | 'large';
  
  /** Whether the button is disabled and non-interactive */
  disabled?: boolean;
  
  /** Callback function triggered when the button is clicked */
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;
}

export default ButtonProps;