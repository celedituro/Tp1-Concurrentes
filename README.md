# CoffeeGPT

## Análisis del Problema

![Tp-Concu](https://user-images.githubusercontent.com/67125933/232071325-91781e50-cf5c-4397-bff5-455284c109cf.png)

## Hipótesis

- Cada dispensador de una máquina puede preparar un café.
- Los N dispensadores de una máquina actúan concurrentemente.
- Un dispenser puede cargar no siempre el mismo ingrediente.
- Los contenedores empiezan llenos.
- Los contenedores no se recargan.
- Un pedido tiene los 4 ingredientes pero el valor de alguno de los mismos puede ser 0.

## Ejecución del programa

```cargo run orders.json```

## Dependencias

- serde para deserializar el archivo de pedidos.

## Resolución del problema

Tengo 1 máquina de café con N dispensers y N ordenes. Los dispensers hacen 1 orden y de forma simulatánea. Para hacer 1 orden, los dispensers le piden los ingredientes a los contenedores de la máquina de café. Por ahora, el orden en que los dispensers piden café está preestablecido: café, agua, cacao y espuma.
