# Uniter

Uniter is a unit converter CLI tool.

![Alt](https://repobeats.axiom.co/api/embed/2ab893442b8a40960cd9bb66e91fdd1986356e69.svg "Repobeats analytics image")



<details>
  <summary><h2>🧰 Commands</h2></summary>

  Convertion Types
  <ul>
  <li>Temperature(Celsius, Fahrenheit, Kelvin)</li>
  <li>Weight(Metric, Imperial)</li>
  <li>Lenght(Metric, Imperial, Space)</li>
  <li>Money(All currencies under FXRatesAPI, also you don't need any API keys)</li>
  <li>Time</li>
  <li>Volume(Metric, Imperial, Solid, Liquid)</li>
  </ul>

  Other commands
  <ul>
  <li>face - Enters interface/dialog option.
  <li>help
  <li>exit
  </ul>

  ## Other info

  Money maximum decimals is 15 due to the API.

  In the decimal rounding it will return 0.0 only if the value is truly close to zero, not just small.

  ## Example usage

  Input: "convert_initial" "value""input_unit" "output_unit" "decimals"

  Input:
  ```
  w 98kg sh.t 8
  ```
  Output:
  ```
  0.10802651sh.t
  ```

  Input:
  ```
  m 89eur jpy
  ```
  Output:
  ```
  14421.6021405JPY
  ```

</details>

<details>
  <summary><h2>🚀 Running Uniter</h2></summary>

Start cloning Uniter:

```
git clone https://github.com/staxhinho/Uniter.git
```


To compile and run:

```
cargo run
```

Make sure you have cargo installed and you're inside the directory in your terminal.
</details>

<details>
  <summary><h2>⚙️ Development</h2></summary>

# Follow commit template

## Commit types

<ul>
  <li> New - New feature.
  <li> Fix - Fixed or optimized a feature.
  <li> Feat - Changed something in a feature that is not fixing, like changing logic or adding new behaviour.
  <li> Docs - Changing documentation.
</ul>

# Message examples

### Commit message with description and breaking change footer
Feat: allow provided config object to extend other configs

BREAKING CHANGE: 'extends' key in config file is now used for extending other config files.

### Commit message with no body
Fix: correct spelling of CHANGELOG
</details>