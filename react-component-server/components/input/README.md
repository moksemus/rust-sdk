# Input Component

A text input field component from Fluent UI that allows users to enter and edit text with various appearances and validation states.

## Installation

```bash
npm install @fluentui/react-components
```

## Import

```typescript
import { Input } from './components/input/component';
// or
import Input from './components/input/component';
```

## Usage

### Basic Input
```tsx
<Input placeholder="Enter your name" />
```

### Controlled Input
```tsx
const [value, setValue] = useState('');

<Input 
  value={value}
  onChange={(e) => setValue(e.target.value)}
  placeholder="Controlled input"
/>
```

### Email Input
```tsx
<Input 
  type="email"
  placeholder="Enter your email"
/>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string` | - | The controlled value of the input |
| `placeholder` | `string` | - | Placeholder text when input is empty |
| `type` | `'text' \| 'email' \| 'password' \| 'tel' \| 'url'` | `'text'` | The type of input field |
| `size` | `'small' \| 'medium' \| 'large'` | `'medium'` | The size of the input field |
| `appearance` | `'outline' \| 'underline' \| 'filled-darker' \| 'filled-lighter'` | `'outline'` | The visual appearance |
| `disabled` | `boolean` | `false` | Whether the input is disabled |
| `onChange` | `(event: React.ChangeEvent<HTMLInputElement>) => void` | - | Change event handler |

## Examples

See the `examples/` directory for more detailed usage examples.

## Accessibility

The Input component follows WCAG guidelines and includes:
- Proper label association
- Keyboard navigation support
- Screen reader compatibility
- Focus management
- High contrast support