// main.js
import init, { make_town_equipment } from "./pkg/arm_create_simulator.js";

async function createWeapon() {
  await init(); // WASM 모듈 초기화

  const equipmentType = parseInt(document.getElementById("equipmentType").value);
  const weaponType = parseInt(document.getElementById("weaponType").value);
  const damagePrice = parseInt(document.getElementById("damagePrice").value);
  const weightPrice = parseInt(document.getElementById("weightPrice").value);
  const weaponElement = parseInt(document.getElementById("weaponElement").value);
  const townElement = parseInt(document.getElementById("townElement").value);
  const guildElement = parseInt(document.getElementById("guildElement").value);

  const equipment = make_town_equipment(
    equipmentType,
    weaponType,
    damagePrice,
    weightPrice,
    weaponElement,
    townElement,
    guildElement
  );

  // 결과 출력
  document.getElementById("result-power").textContent = equipment.damage;
  document.getElementById("result-weight").textContent = equipment.weight;
  document.getElementById("result-price").textContent = equipment.price;

  document.getElementById("result").style.display = "block";
}

window.createWeapon = createWeapon;