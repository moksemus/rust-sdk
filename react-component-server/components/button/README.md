# Button Component

A versatile button component from Fluent UI that supports various appearances, sizes, and states for triggering actions in applications.

## Installation

```bash
npm install @fluentui/react-components
```

## Import

```typescript
import { Button } from './components/button/component';
// or
import Button from './components/button/component';
```

## Usage

### Basic Button
```tsx
<Button>Click me</Button>
```

### Primary Button
```tsx
<Button appearance="primary">
  Primary Action
</Button>
```

### Button with Click Handler
```tsx
<Button 
  appearance="outline"
  onClick={() => console.log('Button clicked!')}
>
  Click Handler
</Button>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `React.ReactNode` | - | The content to display inside the button |
| `appearance` | `'primary' \| 'outline' \| 'subtle' \| 'secondary' \| 'transparent'` | `'secondary'` | The visual appearance style |
| `shape` | `'rounded' \| 'circular' \| 'square'` | `'rounded'` | The shape styling |
| `size` | `'small' \| 'medium' \| 'large'` | `'medium'` | The size of the button |
| `disabled` | `boolean` | `false` | Whether the button is disabled |
| `onClick` | `(event: React.MouseEvent<HTMLButtonElement>) => void` | - | Click event handler |

## Examples

See the `examples/` directory for more detailed usage examples.

## Accessibility

The Button component follows WCAG guidelines and includes:
- Proper focus management
- Keyboard navigation support
- Screen reader compatibility
- High contrast support