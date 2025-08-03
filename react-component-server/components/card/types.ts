import { CardProps as FluentCardProps } from '@fluentui/react-components';

/**
 * Props interface for the Card component
 * Extends Fluent UI CardProps with additional customizations
 */
export interface CardProps extends Omit<FluentCardProps, 'as'> {
  /** The content to display inside the card */
  children?: React.ReactNode;
  
  /** The visual appearance of the card */
  appearance?: 'filled' | 'filled-alternative' | 'outline' | 'subtle';
  
  /** The size of the card */
  size?: 'small' | 'medium' | 'large';
  
  /** The layout orientation of the card */
  orientation?: 'horizontal' | 'vertical';
}

export default CardProps;