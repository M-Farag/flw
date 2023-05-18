### FLW
Process text using configurable tasks.
The CLI app can read tasks from any .yml file that follow the task schema.

### Task Schema
```yaml
tasks:
  - data:
      - replace # command to replace text
      - original # text to replace
      - replace text # text to replace with
```

### Available Commands [WIP: more to come]
- replace
- remove
- count


### Usage
```bash
flw -f <path to .yml file> -i <path to input file> -o <path to output file>
```