# CoffeeGPT

## Análisis del Problema

![Tp-Concu](https://user-images.githubusercontent.com/67125933/232071325-91781e50-cf5c-4397-bff5-455284c109cf.png)

## Hipótesis

- Cada dispensador de una máquina puede preparar un café.
- Los N dispensadores de una máquina actúan concurrentemente.
- Un dispenser puede cargar no siempre el mismo ingrediente.
- Los contenedores empiezan llenos.
- Los contenedores no se recargan.
- Un pedido tiene los 4 ingredientes pero el valor de algunos de los 4 puede ser 0.

## Ejecución del programa

```cargo run orders.json```

## Dependencias

- serde para serializar el archivo de pedidos.
