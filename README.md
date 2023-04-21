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
- Las ordenes se obtienen al principio de la ejecución del programa a partir de un archivo json.

## Ejecución del programa

```cargo run orders.json```

## Dependencias

- serde para deserializar el archivo de pedidos.

## Resolución del problema

Hay N máquinas de café con N dispensers y 4 contenedores cada una. Los dispensers de una máquina de café va a hacer ordenes hasta que no haya más ordenes para procesar y de forma simulatánea con el resto de los dispensers de la misma máquina. Para hacer cada orden, los dispensers le piden los ingredientes a los contenedores de su máquina de café. El orden en que los dispensers piden café está preestablecido: café, agua, cacao y espuma.
