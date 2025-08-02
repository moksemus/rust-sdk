use std::collections::HashMap;
use crate::server::types::{ReactComponent, ComponentProp, ComponentExample, Documentation, DocumentationSection};

pub fn get_sample_components() -> HashMap<String, ReactComponent> {
    let mut components = HashMap::new();

    // Button Component
    components.insert("Button".to_string(), ReactComponent {
        name: "Button".to_string(),
        description: "A customizable button component with various styles and sizes".to_string(),
        source_code: r#"import React from 'react';
import './Button.css';

interface ButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'small' | 'medium' | 'large';
  disabled?: boolean;
  onClick?: () => void;
  className?: string;
}

export const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'medium',
  disabled = false,
  onClick,
  className = '',
}) => {
  const baseClasses = 'btn';
  const variantClass = `btn--${variant}`;
  const sizeClass = `btn--${size}`;
  const disabledClass = disabled ? 'btn--disabled' : '';
  
  const classes = [baseClasses, variantClass, sizeClass, disabledClass, className]
    .filter(Boolean)
    .join(' ');

  return (
    <button
      className={classes}
      disabled={disabled}
      onClick={onClick}
      type="button"
    >
      {children}
    </button>
  );
};

export default Button;"#.to_string(),
        props: vec![
            ComponentProp {
                name: "children".to_string(),
                prop_type: "React.ReactNode".to_string(),
                required: true,
                default_value: "".to_string(),
                description: "The content to display inside the button".to_string(),
            },
            ComponentProp {
                name: "variant".to_string(),
                prop_type: "'primary' | 'secondary' | 'danger'".to_string(),
                required: false,
                default_value: "'primary'".to_string(),
                description: "The visual style variant of the button".to_string(),
            },
            ComponentProp {
                name: "size".to_string(),
                prop_type: "'small' | 'medium' | 'large'".to_string(),
                required: false,
                default_value: "'medium'".to_string(),
                description: "The size of the button".to_string(),
            },
            ComponentProp {
                name: "disabled".to_string(),
                prop_type: "boolean".to_string(),
                required: false,
                default_value: "false".to_string(),
                description: "Whether the button is disabled".to_string(),
            },
            ComponentProp {
                name: "onClick".to_string(),
                prop_type: "() => void".to_string(),
                required: false,
                default_value: "".to_string(),
                description: "Function to call when button is clicked".to_string(),
            },
        ],
        examples: vec![
            ComponentExample {
                title: "Basic Usage".to_string(),
                description: "A simple primary button".to_string(),
                code: r#"<Button onClick={() => console.log('clicked')}>
  Click me
</Button>"#.to_string(),
                props: HashMap::new(),
            },
            ComponentExample {
                title: "Secondary Button".to_string(),
                description: "A secondary variant button".to_string(),
                code: r#"<Button variant="secondary" size="large">
  Secondary Action
</Button>"#.to_string(),
                props: HashMap::new(),
            },
            ComponentExample {
                title: "Danger Button".to_string(),
                description: "A danger variant for destructive actions".to_string(),
                code: r#"<Button variant="danger" onClick={() => handleDelete()}>
  Delete Item
</Button>"#.to_string(),
                props: HashMap::new(),
            },
        ],
        category: "UI".to_string(),
        tags: vec!["button".to_string(), "interactive".to_string(), "form".to_string()],
        typescript_definitions: Some("export interface ButtonProps { ... }".to_string()),
    });

    // Card Component
    components.insert("Card".to_string(), ReactComponent {
        name: "Card".to_string(),
        description: "A flexible card component for displaying content with optional header and footer".to_string(),
        source_code: r#"import React from 'react';
import './Card.css';

interface CardProps {
  children: React.ReactNode;
  title?: string;
  subtitle?: string;
  footer?: React.ReactNode;
  className?: string;
  elevation?: 'none' | 'low' | 'medium' | 'high';
}

export const Card: React.FC<CardProps> = ({
  children,
  title,
  subtitle,
  footer,
  className = '',
  elevation = 'medium',
}) => {
  const baseClasses = 'card';
  const elevationClass = `card--elevation-${elevation}`;
  const classes = [baseClasses, elevationClass, className]
    .filter(Boolean)
    .join(' ');

  return (
    <div className={classes}>
      {(title || subtitle) && (
        <div className="card__header">
          {title && <h3 className="card__title">{title}</h3>}
          {subtitle && <p className="card__subtitle">{subtitle}</p>}
        </div>
      )}
      <div className="card__content">
        {children}
      </div>
      {footer && (
        <div className="card__footer">
          {footer}
        </div>
      )}
    </div>
  );
};

export default Card;"#.to_string(),
        props: vec![
            ComponentProp {
                name: "children".to_string(),
                prop_type: "React.ReactNode".to_string(),
                required: true,
                default_value: "".to_string(),
                description: "The main content of the card".to_string(),
            },
            ComponentProp {
                name: "title".to_string(),
                prop_type: "string".to_string(),
                required: false,
                default_value: "".to_string(),
                description: "Optional title for the card header".to_string(),
            },
            ComponentProp {
                name: "elevation".to_string(),
                prop_type: "'none' | 'low' | 'medium' | 'high'".to_string(),
                required: false,
                default_value: "'medium'".to_string(),
                description: "The shadow elevation level of the card".to_string(),
            },
        ],
        examples: vec![
            ComponentExample {
                title: "Basic Card".to_string(),
                description: "A simple card with title and content".to_string(),
                code: r#"<Card title="Welcome" subtitle="Getting started">
  <p>This is the main content of the card.</p>
</Card>"#.to_string(),
                props: HashMap::new(),
            },
        ],
        category: "Layout".to_string(),
        tags: vec!["card".to_string(), "container".to_string(), "layout".to_string()],
        typescript_definitions: Some("export interface CardProps { ... }".to_string()),
    });

    // Input Component
    components.insert("Input".to_string(), ReactComponent {
        name: "Input".to_string(),
        description: "A controlled input component with validation and various types".to_string(),
        source_code: r#"import React from 'react';
import './Input.css';

interface InputProps {
  value: string;
  onChange: (value: string) => void;
  type?: 'text' | 'email' | 'password' | 'number';
  placeholder?: string;
  label?: string;
  error?: string;
  disabled?: boolean;
  required?: boolean;
  className?: string;
}

export const Input: React.FC<InputProps> = ({
  value,
  onChange,
  type = 'text',
  placeholder,
  label,
  error,
  disabled = false,
  required = false,
  className = '',
}) => {
  const inputId = React.useId();
  const hasError = Boolean(error);
  
  const inputClasses = [
    'input__field',
    hasError ? 'input__field--error' : '',
    disabled ? 'input__field--disabled' : '',
    className
  ].filter(Boolean).join(' ');

  return (
    <div className="input">
      {label && (
        <label htmlFor={inputId} className="input__label">
          {label}
          {required && <span className="input__required">*</span>}
        </label>
      )}
      <input
        id={inputId}
        type={type}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        required={required}
        className={inputClasses}
        aria-invalid={hasError}
        aria-describedby={hasError ? `${inputId}-error` : undefined}
      />
      {error && (
        <span id={`${inputId}-error`} className="input__error">
          {error}
        </span>
      )}
    </div>
  );
};

export default Input;"#.to_string(),
        props: vec![
            ComponentProp {
                name: "value".to_string(),
                prop_type: "string".to_string(),
                required: true,
                default_value: "".to_string(),
                description: "The current value of the input".to_string(),
            },
            ComponentProp {
                name: "onChange".to_string(),
                prop_type: "(value: string) => void".to_string(),
                required: true,
                default_value: "".to_string(),
                description: "Function called when input value changes".to_string(),
            },
            ComponentProp {
                name: "type".to_string(),
                prop_type: "'text' | 'email' | 'password' | 'number'".to_string(),
                required: false,
                default_value: "'text'".to_string(),
                description: "The type of input field".to_string(),
            },
        ],
        examples: vec![
            ComponentExample {
                title: "Basic Input".to_string(),
                description: "A simple text input with label".to_string(),
                code: r#"<Input
  value={inputValue}
  onChange={setInputValue}
  label="Your Name"
  placeholder="Enter your name"
/>"#.to_string(),
                props: HashMap::new(),
            },
        ],
        category: "Form".to_string(),
        tags: vec!["input".to_string(), "form".to_string(), "validation".to_string()],
        typescript_definitions: Some("export interface InputProps { ... }".to_string()),
    });

    components
}

pub fn get_sample_documentation() -> HashMap<String, Documentation> {
    let mut docs = HashMap::new();

    docs.insert("getting-started".to_string(), Documentation {
        topic: "getting-started".to_string(),
        title: "Getting Started with React Components".to_string(),
        content: "Welcome to our React component library! This guide will help you get started with using our components in your projects.".to_string(),
        sections: vec![
            DocumentationSection {
                id: "installation".to_string(),
                title: "Installation".to_string(),
                content: "Install the component library using npm or yarn:".to_string(),
                code_examples: vec![
                    "npm install @yourorg/react-components".to_string(),
                    "yarn add @yourorg/react-components".to_string(),
                ],
            },
            DocumentationSection {
                id: "usage".to_string(),
                title: "Basic Usage".to_string(),
                content: "Import and use components in your React application:".to_string(),
                code_examples: vec![
                    r#"import { Button, Card, Input } from '@yourorg/react-components';

function App() {
  return (
    <div>
      <Card title="Welcome">
        <Input value="" onChange={() => {}} placeholder="Enter text" />
        <Button>Submit</Button>
      </Card>
    </div>
  );
}"#.to_string(),
                ],
            },
        ],
        examples: vec![
            "Basic component usage".to_string(),
            "Theming and customization".to_string(),
        ],
        related_components: vec!["Button".to_string(), "Card".to_string(), "Input".to_string()],
    });

    docs.insert("theming".to_string(), Documentation {
        topic: "theming".to_string(),
        title: "Theming and Customization".to_string(),
        content: "Learn how to customize the appearance of components using CSS variables and custom themes.".to_string(),
        sections: vec![
            DocumentationSection {
                id: "css-variables".to_string(),
                title: "CSS Variables".to_string(),
                content: "Use CSS custom properties to customize component appearance:".to_string(),
                code_examples: vec![
                    r#":root {
  --btn-primary-bg: #007bff;
  --btn-primary-color: white;
  --card-border-radius: 8px;
  --input-border-color: #ddd;
}"#.to_string(),
                ],
            },
        ],
        examples: vec!["Dark theme setup".to_string(), "Custom color schemes".to_string()],
        related_components: vec!["Button".to_string(), "Card".to_string()],
    });

    docs
}