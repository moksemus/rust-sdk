import React from 'react';
import { Card as FluentCard } from '@fluentui/react-components';
import { CardProps } from './types';

/**
 * Card component based on Fluent UI Card
 * 
 * A container component that groups related content and actions in a flexible 
 * layout surface. Perfect for displaying information in an organized way.
 */
export const Card: React.FC<CardProps> = ({
  children,
  appearance = 'filled',
  size = 'medium',
  orientation = 'vertical',
  ...rest
}) => {
  return (
    <FluentCard
      appearance={appearance}
      size={size}
      orientation={orientation}
      {...rest}
    >
      {children}
    </FluentCard>
  );
};

export default Card;