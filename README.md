# **CoffeeGPT**

## **Análisis del Problema**

![Tp-Concu](https://user-images.githubusercontent.com/67125933/232071325-91781e50-cf5c-4397-bff5-455284c109cf.png)

## **Hipótesis**

- Un dispenser de una máquina de café no puede preparar más de una orden de forma simultanea.
- Los N dispensers de una máquina actúan concurrentemente.
- Los contenedores empiezan llenos y no se recargan.
- Una orden de café contiene los 4 ingredientes.

## **Ejecución del programa**

```cargo run orders.json```

## **Dependencias**

- ```serde``` para deserializar el archivo de pedidos.

## **Problema general**

Hay N máquinas de café con N dispensers y 6 contenedores cada una (café molido, espuma de leche, cacao, agua, café en grano y leche). Los dispensers de una máquina de café van a preparar ordenes hasta que no haya más ordenes para procesar (en la lista de ordenes que se obtiene del archivo que ingresa el usuario). Las ordenes de café son procesadas simultaneamente entre los dispensers de una misma máquina, sin embargo un dispenser no puede procesar más de una orden a la vez. Para hacer cada orden, los dispensers le piden los ingredientes a los contenedores de su máquina de café.

## **Resolución del problema**

Se lanza un thread por cada máquina de café, así como también se lanza un thread por cada dispenser de cada máquina.

### *Reposición de ingredientes*

Esta tarea es llevada a cabo por el objeto IngredientHandler. Por cada máquina de café se dispone de un IngredientHandler que es el que va a chequear y notificar si hay que reponer algún ingrediente así como llamar a los contenedores para que se realice la reposición de los ingredientes correspondientes.

Por cada máquina de café se van a lanzar 3 threads (uno por cada ingrediente que se puede reponer: café, espuma de leche y agua), que van a estar esperando continuamente (hasta que no haya más ordenes que procesar) hasta que se le notifique que tiene que reponer alguno de los 3 ingredientes. Para esto utilicé un mutex junto con una condvar. El mutex es un vector de 3 elementos de tipo bool que representa si hay o que reponer un ingrediente.

Los dispensers son quienes le notifican al ingredient handler cuando es necesario reponer ingredientes.

### *Presentación de estadísticas*

Las estadísticas son realizadas por medio de un thread que va mostrarlas periódicamente hasta que no haya más ordenes que procesar. Para evitar que no se muestren las estadísticas si no se terminó de procesar ninguna orden, utilicé una condvar.

## **Casos de prueba**

Los distintos casos de prueba se encuentran en el directorio /resource y muestran distintas situaciones de la ejecución del programa dependiendo del archivo de pedidos que reciba. Para hacer los casos de prueba se tuvo en cuenta que se dispone de 2 máquinas de café con 3 dispensers cada una, la cantidad inicial
de ingredientes es 100 y los pedidos que se realizan son iguales para facilitar la verificación de los resultados. En particular se utilizó el siguiente ejemplo de pedido: {"coffee": 10, "water": 10, "cocoa": 10, "foam": 10}, de manera que si una máquina realiza 10 pedidos, tiene que reponer ingredientes:

- orders01.json: sin reposición de ingredientes. Se reciben 4 pedidos de manera que si una de las 2 máquinas decide hacer los 4 pedidos, no tiene que reponer ingredientes.
- orders02.json: con reposición de ingredientes. Se reciben 10 pedidos de manera que si una de las 2 máquinas decide hacer más de 5 pedidos, tiene que reponer ingredientes.
- orders03.json: con doble reposición de ingredientes. Se reciben 20 pedidos de manera que si una de las 2 máquinas decide hacer más de 5 pedidos, tiene que reponer ingredientes.
