package main

import gl "vendor:OpenGL"

import "core:fmt"

Entity :: struct {
    position: [3]f32,
    size: [3]f32
}

Card :: struct {
    using entity: Entity,
    suit: i32,
    
}

Card_New :: proc() -> ^Card {
    card := new(Card)
    return card
}

Card_Draw :: proc(card: Card) {
    
}
