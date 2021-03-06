# Full Example

This example uses most of the supported features in flowdown. It walks through
a simple pizzeria customer service experience.

```
Hello! Welcome to Flowdown Pizzaria, what can I do for you?
[capture $mode]
* $mode == "order pizza": -> @order
* $mode == "menu": [image https://flowdownpizza/menu.png]

@ order

    What type of pizza would you like?
    [capture $pizzaType]

    What size of pizza?
    [capture $pizzaSize]

    How would you like to recieve your pizza?
    [capture $pizzaMethod]
    * $pizzaMethod == "delivery": -> @delivery
    * $pizzaMethod == "take out": -> @take out

    Thank you for choosing Flowdown Pizzaria!
    -> @survey

@ delivery

    Can I get an address
    [capture $address]

    [set $price "0"]
    [set $deliveryTime "0"]
    [code calculatePrice.js]
    [code computeDeliveryRoute.js]

    Your final price is {$price} and you will get your pizza in about {$deliveryTime}! 

@ take out

    When would you like to pick up your food?
    [capture $pickupTime]

    [set $price "0"]
    [code calculatePrice.js]

    Your final price is {$price}.

@ survey

    Would you like to complete an optional survey?
    [capture $survey]
    * $survey == "yes": -> start survey
    * $survey == "no": -> end survey

    = start survey

        How would you rate today's experience?
        [capture $rating]

        Is there any feedback you would like to give?
        [capture $feedback]

    = end survey

        Thank you!
```
