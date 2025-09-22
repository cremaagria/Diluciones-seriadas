# Diluciones-seriadas
Programa desarrollado para calcular dilusiones seriadas en una muestra, y calcular la media, la desviacion estandar, el r y el r².

## Cálculo de la correlación de Pearson
La fórmula general es $\frac{1}{2}$:

$$
r = \frac{1}{n-1}\sum_{i=1}^n \left( \frac{x_i - \bar{x}}{s_x} \right)\left( \frac{y_i - \bar{y}}{s_y} \right)
$$


Donde:

- $n$ : número de observaciones.  
- $ x_i, y_i$ : valores individuales de las variables x y y.  
- $ \bar{x}  = \frac{1}{n}\sum_{i=1}^n x_i $: media de x.  
- $ \bar{y} = \frac{1}{n}\sum_{i=1}^n y_i $: media de y.  
- $ s_x $: desviación estándar muestral de x.  
- $ s_y $: desviación estándar muestral de y.  

Las desviaciones estándar se definen como:

$$
s_x = \sqrt{\frac{1}{n-1}\sum_{i=1}^n (x_i - \bar{x})^2}
$$

Las medias se defienen como 
$$
 \frac{1}{n}\sum_{i=1}^n x_i
$$

---

En resumen, la media se obtiene como:

$$
\bar{x} = \frac{\sum x_i}{n}, \quad \bar{y} = \frac{\sum y_i}{n}
$$

Asi mismo se calcula el coheficiente de determinación el cual es $r²$.
