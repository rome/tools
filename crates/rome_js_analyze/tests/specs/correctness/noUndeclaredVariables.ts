// valid
export type EventHandler<T extends string> = `on${T}`
export type EventHandlerDefault<T extends string = 'click'> = `on${T}`

export type NestedContext<S extends NestedContext<''>> = '' | `(${S})`
export type NestedContextDefault<S extends NestedContextDefault = ''> = '' | `(${S})`

export type Whatever<S extends number> = `Hello ${S}`
export type WhateverDefault<S extends number = 2> = `Hello ${S}`

// Invalid
export type Invalid<S extends number> = `Hello ${T}`