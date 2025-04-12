/* tslint:disable */
/* eslint-disable */
export function make_town_equipment(equipment_type: EquipmentType, weapon_type: WeaponType, damage_price: number, weigh_price: number, weapon_element: ElementType, creator_element: ElementType, contry_element: ElementType): any;
export enum ElementType {
  Fire = 1,
  Wind = 2,
  Star = 3,
  Thunder = 4,
  Water = 5,
  Light = 6,
  Dark = 7,
}
export enum EquipmentType {
  Weapon = 1,
  Armor = 2,
  Accessory = 3,
}
export enum WeaponType {
  Sword = 1,
  Spear = 2,
  Axe = 3,
  Knuckle = 4,
  Bow = 5,
  MagicWand = 6,
  Knife = 7,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly make_town_equipment: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
