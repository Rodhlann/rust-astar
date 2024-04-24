# rust-astar
A* Search Algorithm written in Rust

## Example

This example output shows the resolved path with parameters:

- **Start**: Cell [8, 3]
- **End**: Cell [6, 3]
- **Walls**: Represented by the 9s in this example output

The algorithm is able to rapidly detect how to find the end cell in the "room"
in under 1ms. 

```
[
 [ ,  ,  , 6, 5, 4, 3, 4, 5,  ],
 [ ,  ,  , 5, 9, 9, 9, 9, 4,  ],
 [ ,  ,  , 4, 9,  ,  , 9, 3,  ],
 [ ,  ,  , 3, 2, 1,  , 9, 2,  ],
 [ ,  ,  ,  , 9,  ,  , 9,  ,  ],
 [ ,  ,  ,  , 9, 9, 9, 9,  ,  ],
 [ ,  ,  ,  ,  ,  ,  ,  ,  ,  ],
 [ ,  ,  ,  ,  ,  ,  ,  ,  ,  ],
 [ ,  ,  ,  ,  ,  ,  ,  ,  ,  ],
 [ ,  ,  ,  ,  ,  ,  ,  ,  ,  ]
]
```

## Purpose

I had experienced issues building a similar function 
in JavaScript, and wanted to see if I could better understand
the implementation by building it another language. 

I would like to compile WASM in the future, fix the JS 
implementation, and compare performance. 
