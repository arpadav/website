# notes

## Diff EMA Risk

$$
R(t_{n - \nu}) = - |x(t_n)| - \left( \frac{dx(t_n)}{dt} \right)^{2}
$$

where

$$
x(t_n) = EMA_{p}(t_n) - EMA_{q}(t_{n})
$$

where $p > q$, and immediate risk is where $\nu = 0$ and predicted risk of historical data is where $\nu > 0$
