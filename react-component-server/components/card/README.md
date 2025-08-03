# Card Component

A container component from Fluent UI that groups related content and actions in a flexible layout surface.

## Installation

```bash
npm install @fluentui/react-components
```

## Import

```typescript
import { Card } from './components/card/component';
// or
import Card from './components/card/component';
```

## Usage

### Basic Card
```tsx
<Card>
  <div style={{ padding: '16px' }}>
    <h3>Card Title</h3>
    <p>Card content goes here.</p>
  </div>
</Card>
```

### Outline Card
```tsx
<Card appearance="outline">
  <div style={{ padding: '16px' }}>
    <h3>Outline Card</h3>
    <p>This card has a border outline.</p>
  </div>
</Card>
```

### Different Sizes
```tsx
<Card size="large" appearance="filled">
  <div style={{ padding: '20px' }}>
    <h2>Large Card</h2>
    <p>More spacious content area.</p>
  </div>
</Card>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `React.ReactNode` | - | The content to display inside the card |
| `appearance` | `'filled' \| 'filled-alternative' \| 'outline' \| 'subtle'` | `'filled'` | The visual appearance |
| `size` | `'small' \| 'medium' \| 'large'` | `'medium'` | The size of the card |
| `orientation` | `'horizontal' \| 'vertical'` | `'vertical'` | The layout orientation |

## Examples

See the `examples/` directory for more detailed usage examples.

## Best Practices

- Use cards to group related information
- Keep card content concise and scannable
- Use appropriate spacing and padding
- Consider card hierarchy and nesting carefully