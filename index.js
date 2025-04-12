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
  document.getElementById("result-power").textContent = equipment.damage.value;
  document.getElementById("result-power-min").textContent = equipment.damage.min;
  document.getElementById("result-power-max").textContent = equipment.damage.max;

  document.getElementById("result-weight").textContent = equipment.weight.value;
  document.getElementById("result-weight-min").textContent = equipment.weight.min;
  document.getElementById("result-weight-max").textContent = equipment.weight.max;
  
  document.getElementById("result-price").textContent = equipment.price.value.toLocaleString();
  document.getElementById("result-price-min").textContent = equipment.price.min.toLocaleString();
  document.getElementById("result-price-max").textContent = equipment.price.max.toLocaleString();

  document.getElementById("result").style.display = "block";
}

window.createWeapon = createWeapon;