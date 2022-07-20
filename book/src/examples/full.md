# Full Example

```
@ welcome

    {good day|welcome back $name|how are you doing $name}!
    welcome to my store!  # excited

    = self introduction

        my name is pinosaur

        skip to any part of my introduction
        * what have i been doing                        -> backstory
        * what i am currently doing                     -> present day
        * what i will do in the future                  -> future
        * store introduction (skip this entire section) -> store introduction

        == backstory
        i have been making stores for over 20 years

        == present day
        i am currently making a store

        == future

        i will continue making stores

    now let's get to the store's introduciton!!!

    = store introduction

        my store is called the store of stores

    what would you like to do now?
    * go to shop -> @shop
    * contact us -> @contact

    welcome back (return from sub-topic)

@ shop

    todays items for sale
    * monday menu -> monday
    * tuesday menu -> tuesday
    * wednesday menu -> wednesday

    = monday
    burgers -> done

    = tuesday
    ramen -> done

    = wednesday
    pasta -> done

    = done

@ contact

```
