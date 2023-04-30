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

Se lanza un thread por cada máquina de café, así como también se lanza un thread por cada dispenser de cada máquina. Los dispensers de las máquinas van a tomar ordenes de la lista de ordenes, van a pedirle los ingredientes a los contenedores correspondientes y van a seguir armando ordenes hasta que no haya más ordenes por procesar.

Si un dispenser no logra obtener un ingrediente de la orden porque el contenedor del mismo no tiene esa cantidad de ingrediente disponible, va a notificarle al thread que se encarga de su reposición y va a intentar obtener ese ingrediente nuevamente. Si el ingrediente no puede ser repuesto porque no hay recurso suficiente, la orden no se completa y los ingredientes de la misma que ya se hayan obtenido se van a descartar.

### *Reposición de ingredientes*

Esta tarea es llevada a cabo por el objeto IngredientHandler. Cada máquina de café tiene un IngredientHandler que es el que va a realizar la reposición de los ingredientes.

Por cada máquina de café se van a lanzar 3 threads (uno por cada ingrediente que se puede reponer: café, espuma de leche y agua). Estos threads van a estar esperando continuamente (hasta que no haya más ordenes que procesar) hasta que se le notifique que tienen que reponer alguno de los 3 ingredientes. Para esto utilicé un mutex junto con una condvar. El mutex es un vector de 3 elementos de tipo bool que representa si hay o que reponer un ingrediente.

### *Presentación de estadísticas*

Las estadísticas son realizadas por medio de un thread que va obtenerlas y mostrarlas periódicamente hasta que no haya más ordenes que procesar. Para evitar que no se muestren las estadísticas si no se terminó de procesar ninguna orden, utilicé una condvar.

## **Casos de prueba**

Los distintos casos de prueba se encuentran en el directorio /resource y muestran distintas situaciones de la ejecución del programa dependiendo del archivo de pedidos que reciba.

Para hacer los casos de prueba se tuvieron en cuenta las siguientes consideraciones de manera de facilitar la verificación de los resultados:

- Se disponen de 2 máquinas de café.
- La cantidad inicial de ingredientes es 100.
- La cantidad de ingrediente que se repone es 50.
- Los pedidos que se realizan son iguales con el objetivo de facilitar. En particular se utilizó el pedido: {"coffee": 10, "water": 10, "cocoa": 1, "foam": 10}. La cantidad de cocoa es mínima porque no se puede reponer. Por lo tanto si una máquina realiza 10 pedidos, tiene que reponer ingredientes y si procesa 15 pedidos va a tener que hacer otra reposición de ingredientes para poder realizar el próximo.

- orders01.json: sin reposición de ingredientes. Se reciben 8 pedidos de manera que si una de las 2 máquinas decide hacer todos pedidos, no tiene que reponer ingredientes.
- orders02.json: con reposición de ingredientes. Se reciben 21 pedidos de manera que si o sí una de las áquinas tiene que reponer ingredientes.
- orders03.json: con doble reposición de ingredientes. Se reciben 32 pedidos para asegurarnos de que las 2 máquinas hagan doble reposición.
- orders04.json: agotamiento de ingredientes. Se reciben 45 pedidos para asegurarnos de que las 2 máquinas agoten sus recursos.
