<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type NavItem = {
  id: string;
  label: string;
  disabled?: boolean;
};


type ApiPerson = {
  id: number;
  last_name: string;
  first_name: string;
  middle_name: string;
  sex: string;
  birth_date: string;
  birth_place: string;
  father: string;
  mother: string;
  residence: string;
  occupation: string;
  legitimacy: string;
  midwife: string;
  godparents: string;
  priest: string;
  notes: string;
  created_at: string;
};

type PersonRow = {
  id: number;
  name: string;
  life: string;
  place: string;
  source: string;
};

const navItems: NavItem[] = [
  { id: "stats", label: "Статистика" },
  { id: "persons", label: "Особи" },
  { id: "places", label: "Місця" },
  { id: "sources", label: "Джерела" },
  { id: "records", label: "Записи", disabled: true },
  { id: "families", label: "Родини", disabled: true },
  { id: "trees", label: "Дерева", disabled: true },
  { id: "imports", label: "Імпорт / експорт", disabled: true },
  { id: "settings", label: "Налаштування", disabled: true },
];

const activeNav = ref<NavItem["id"]>("persons");
const showAdvanced = ref(false);
const showModal = ref(false);
const modalPosition = reactive({ x: 0, y: 0 });
const isDragging = ref(false);
const dragOffset = reactive({ x: 0, y: 0 });
const modalWidth = 720;

const newPerson = reactive({
  lastName: "",
  firstName: "",
  middleName: "",
  sex: "невідомо",
  birthDate: "",
  birthPlace: "",
  father: "",
  mother: "",
  residence: "",
  occupation: "",
  legitimacy: "",
  midwife: "",
  godparents: "",
  priest: "",
  notes: "",
});

const people = ref<PersonRow[]>([]);

const isSaving = ref(false);
const isLoading = ref(false);
const backendAvailable = ref(false);
let tempId = 0;

const buildPersonRow = (person: ApiPerson): PersonRow => ({
  id: person.id,
  name: [person.last_name, person.first_name, person.middle_name]
    .filter(Boolean)
    .join(" "),
  life: person.birth_date ? `Народж.: ${person.birth_date}` : "Без дати",
  place: person.birth_place || "Без місця",
  source: person.notes ? "Є примітка" : "Без приміток",
});

const buildRowFromForm = (): PersonRow => {
  tempId += 1;
  const name = [newPerson.lastName, newPerson.firstName, newPerson.middleName]
    .filter(Boolean)
    .join(" ");
  return {
    id: -tempId,
    name: name || "Без імені",
    life: newPerson.birthDate ? `Народж.: ${newPerson.birthDate}` : "Без дати",
    place: newPerson.birthPlace || "Без місця",
    source: newPerson.notes ? "Є примітка" : "Без приміток",
  };
};

const resetForm = () => {
  newPerson.lastName = "";
  newPerson.firstName = "";
  newPerson.middleName = "";
  newPerson.sex = "невідомо";
  newPerson.birthDate = "";
  newPerson.birthPlace = "";
  newPerson.father = "";
  newPerson.mother = "";
  newPerson.residence = "";
  newPerson.occupation = "";
  newPerson.legitimacy = "";
  newPerson.midwife = "";
  newPerson.godparents = "";
  newPerson.priest = "";
  newPerson.notes = "";
};

const loadPeople = async () => {
  isLoading.value = true;
  try {
    if (!backendAvailable.value) {
      people.value = [];
      return;
    }
    const result = await invoke<ApiPerson[]>("list_people");
    people.value = result.map(buildPersonRow);
  } catch (error) {
    console.error(error);
  } finally {
    isLoading.value = false;
  }
};

const savePerson = async () => {
  if (isSaving.value) return;
  isSaving.value = true;
  try {
    if (!backendAvailable.value) {
      people.value = [buildRowFromForm(), ...people.value];
      resetForm();
      showAdvanced.value = false;
      return;
    }
    const payload = {
      last_name: newPerson.lastName,
      first_name: newPerson.firstName,
      middle_name: newPerson.middleName,
      sex: newPerson.sex,
      birth_date: newPerson.birthDate,
      birth_place: newPerson.birthPlace,
      father: newPerson.father,
      mother: newPerson.mother,
      residence: newPerson.residence,
      occupation: newPerson.occupation,
      legitimacy: newPerson.legitimacy,
      midwife: newPerson.midwife,
      godparents: newPerson.godparents,
      priest: newPerson.priest,
      notes: newPerson.notes,
    };
    const created = await invoke<ApiPerson>("create_person", { person: payload });
    people.value = [buildPersonRow(created), ...people.value];
    resetForm();
    showAdvanced.value = false;
  } catch (error) {
    console.error(error);
  } finally {
    isSaving.value = false;
  }
};

const openModal = () => {
  if (typeof window !== "undefined") {
    const centeredX = Math.max(24, (window.innerWidth - modalWidth) / 2);
    modalPosition.x = Math.floor(centeredX);
    modalPosition.y = 64;
  }
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
};

const startDrag = (event: MouseEvent) => {
  if (!showModal.value) return;
  isDragging.value = true;
  dragOffset.x = event.clientX - modalPosition.x;
  dragOffset.y = event.clientY - modalPosition.y;
  event.preventDefault();
};

const onDrag = (event: MouseEvent) => {
  if (!isDragging.value) return;
  modalPosition.x = Math.max(12, event.clientX - dragOffset.x);
  modalPosition.y = Math.max(12, event.clientY - dragOffset.y);
};

const stopDrag = () => {
  isDragging.value = false;
};

const modalStyle = computed(() => ({
  transform: `translate(${modalPosition.x}px, ${modalPosition.y}px)`,
}));

onMounted(() => {
  backendAvailable.value =
    typeof window !== "undefined" &&
    (window as { __TAURI_INTERNALS__?: { invoke?: unknown } })
      .__TAURI_INTERNALS__?.invoke !== undefined;
  loadPeople();
  window.addEventListener("mousemove", onDrag);
  window.addEventListener("mouseup", stopDrag);
});

onBeforeUnmount(() => {
  window.removeEventListener("mousemove", onDrag);
  window.removeEventListener("mouseup", stopDrag);
});
</script>

<template>
  <div class="app">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-mark">
          <img
            class="brand-icon"
            src="/src/assets/branding/dzherelo-logo.png"
            alt="Джерело"
          />
        </div>
        <div>
          <div class="brand-title"></div>
        </div>
      </div>

      <nav class="nav">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: activeNav === item.id, disabled: item.disabled }"
          :disabled="item.disabled"
          @click="activeNav = item.id"
        >
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </nav>

      <div class="sidebar-footer">
        <div class="status">
          <span class="status-dot"></span>
          Локальна база: активна
        </div>
      </div>
    </aside>

    <div class="workspace">
      <header class="topbar">
        <div class="topbar-left">
          <h1>Особи</h1>
          <p class="subtitle">Швидке додавання та перегляд основних даних.</p>
        </div>
        <div class="topbar-actions">
          <input class="search" placeholder="Пошук по імені, місцю, джерелу" />
          <button class="btn primary" @click="openModal">
            Нова особа
          </button>
        </div>
      </header>

      <section class="content">
        <div class="panel list-panel">
          <div v-if="!backendAvailable" class="list-empty">
            Запущено у браузері. Для збереження у БД запусти через
            <strong>npm run tauri dev</strong>.
          </div>

          <div v-if="isLoading" class="list-empty">
            Завантаження списку...
          </div>
          <div v-else-if="people.length === 0" class="list-empty">
            Поки що немає записів.
          </div>
          <div v-else class="list">
            <div
              v-for="(person, index) in people"
              :key="person.id"
              class="list-item"
              :style="{ '--delay': `${index * 60}ms` }"
            >
              <div class="list-main">
                <div class="list-name">{{ person.name }}</div>
                <div class="list-sub">{{ person.life }} · {{ person.place }}</div>
              </div>
              <div class="list-source">{{ person.source }}</div>
              <button class="btn ghost">Відкрити</button>
            </div>
          </div>
        </div>
      </section>
    </div>

    <div v-show="showModal" class="modal-wrapper">
      <div class="modal" :style="modalStyle">
        <div class="panel-header modal-handle" @mousedown="startDrag">
          <div>
            <h2>Нова особа</h2>
            <p class="muted">
              Основні поля — одразу. Додаткові — згорнуті.
            </p>
          </div>
          <div class="modal-actions">
            <button
              class="btn ghost"
              type="button"
              @click="showAdvanced = !showAdvanced"
            >
              {{ showAdvanced ? "Сховати деталі" : "Показати деталі" }}
            </button>
            <button class="btn ghost" type="button" @click="closeModal">
              Закрити
            </button>
          </div>
        </div>

          <form class="form-grid">
            <div class="field">
              <label>Прізвище</label>
              <input v-model="newPerson.lastName" placeholder="Шевченко" />
            </div>
            <div class="field">
              <label>Ім'я</label>
              <input v-model="newPerson.firstName" placeholder="Іван" />
            </div>
            <div class="field">
              <label>По батькові</label>
              <input v-model="newPerson.middleName" placeholder="Іванович" />
            </div>
            <div class="field">
              <label>Стать</label>
              <select v-model="newPerson.sex">
                <option value="чоловік">чоловік</option>
                <option value="жінка">жінка</option>
                <option value="невідомо">невідомо</option>
              </select>
            </div>
            <div class="field">
              <label>Дата народження</label>
              <input v-model="newPerson.birthDate" placeholder="1891-04-21" />
            </div>
            <div class="field">
              <label>Місце народження</label>
              <input v-model="newPerson.birthPlace" placeholder="с. Буди" />
            </div>
            <div class="field">
              <label>Батько</label>
              <input v-model="newPerson.father" placeholder="ПІБ або ID" />
            </div>
            <div class="field">
              <label>Мати</label>
              <input v-model="newPerson.mother" placeholder="ПІБ або ID" />
            </div>
          </form>

          <transition name="reveal">
            <div v-if="showAdvanced" class="advanced-grid">
              <div class="field">
                <label>Місце проживання</label>
                <input v-model="newPerson.residence" placeholder="с. Високе" />
              </div>
              <div class="field">
                <label>Заняття батьків</label>
                <input v-model="newPerson.occupation" placeholder="хлібороб" />
              </div>
              <div class="field">
                <label>Законність</label>
                <input v-model="newPerson.legitimacy" placeholder="шлюбний" />
              </div>
              <div class="field">
                <label>Повитуха</label>
                <input v-model="newPerson.midwife" placeholder="ПІБ" />
              </div>
              <div class="field">
                <label>Хрещені / свідки</label>
                <input v-model="newPerson.godparents" placeholder="ПІБ" />
              </div>
              <div class="field">
                <label>Священик</label>
                <input v-model="newPerson.priest" placeholder="ПІБ" />
              </div>
              <div class="field full">
                <label>Примітки</label>
                <textarea
                  v-model="newPerson.notes"
                  placeholder="Короткі уточнення або посилання на джерела."
                ></textarea>
              </div>
            </div>
          </transition>

          <div class="form-actions">
            <button class="btn ghost" type="button" @click="resetForm">
              Скасувати
            </button>
            <button class="btn primary" type="button" @click="savePerson">
              {{ isSaving ? "Збереження..." : "Зберегти" }}
            </button>
          </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
:root {
  color-scheme: light;
  font-family: "Space Grotesk", "Manrope", "Segoe UI", sans-serif;
  line-height: 1.35;
  font-size: 14px;

  --bg: #f8f5ef;
  --bg-contrast: #efe7dc;
  --panel: #ffffff;
  --ink: #1e1a17;
  --muted: #6f6256;
  --accent: #e07a3f;
  --accent-strong: #bf5e23;
  --line: #e3d7ca;
  --shadow: 0 14px 40px rgba(45, 31, 20, 0.12);
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  background:
    radial-gradient(circle at top right, #f1d7c8 0%, transparent 55%),
    linear-gradient(135deg, #f8f5ef 0%, #f2ece4 45%, #f6efe6 100%);
  color: var(--ink);
  min-height: 100vh;
}

.app {
  display: grid;
  grid-template-columns: 190px 1fr;
  min-height: 100vh;
  position: relative;
}

.sidebar {
  padding: 16px 12px;
  border-right: 1px solid var(--line);
  background: linear-gradient(180deg, #fff6ee 0%, #f3e8dc 100%);
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.brand {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 14px;
}

.brand-mark {
  width: 102px;
  height: 102px;
  border-radius: 16px;
  background: var(--accent);
  color: #fff;
  font-weight: 700;
  font-size: 18px;
  display: grid;
  place-items: center;
  box-shadow: 0 8px 18px rgba(224, 122, 63, 0.35);
  overflow: hidden;
}

.brand-icon {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.brand-title {
  font-weight: 700;
  font-size: 14px;
}

.brand-sub {
  font-size: 10px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--muted);
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 10px;
  border-radius: 9px;
  border: 1px solid rgba(227, 215, 202, 0.8);
  background: rgba(255, 255, 255, 0.4);
  color: inherit;
  font-weight: 500;
  font-size: 12px;
  text-align: left;
  cursor: pointer;
  transition: transform 0.2s ease, background 0.2s ease, border 0.2s ease;
  position: relative;
}

.nav-item:hover:not(.disabled) {
  background: rgba(255, 255, 255, 0.7);
  border-color: rgba(224, 122, 63, 0.2);
  transform: translateX(4px);
}

.nav-item.active {
  background: #fff;
  border-color: rgba(224, 122, 63, 0.35);
  box-shadow: 0 6px 14px rgba(224, 122, 63, 0.12);
  font-weight: 600;
}

.nav-item.active::before {
  content: "";
  position: absolute;
  left: -6px;
  top: 8px;
  bottom: 8px;
  width: 3px;
  border-radius: 999px;
  background: var(--accent);
}

.nav-item.disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.nav-badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 999px;
  background: rgba(224, 122, 63, 0.15);
  color: var(--accent-strong);
}

.sidebar-footer {
  margin-top: auto;
  font-size: 11px;
  color: var(--muted);
}

.status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #3fa34d;
  box-shadow: 0 0 0 4px rgba(63, 163, 77, 0.2);
}

.workspace {
  display: flex;
  flex-direction: column;
  padding: 16px 18px 22px;
  gap: 12px;
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
}

.topbar h1 {
  margin: 0 0 4px;
  font-size: 18px;
}

.subtitle {
  margin: 0;
  color: var(--muted);
  font-size: 12px;
}

.topbar-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.search {
  min-width: 180px;
  padding: 6px 10px;
  border-radius: 9px;
  border: 1px solid var(--line);
  background: #fff;
}

.content {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
}

.panel {
  background: var(--panel);
  border-radius: 14px;
  padding: 12px;
  box-shadow: var(--shadow);
  animation: fadeIn 0.6s ease forwards;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
  margin-bottom: 8px;
}

.muted {
  color: var(--muted);
}

.form-grid {
  display: grid;
  gap: 8px;
}

.advanced-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px;
  margin-top: 8px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field.full {
  grid-column: 1 / -1;
}

label {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--muted);
}

input,
select,
textarea {
  border-radius: 9px;
  border: 1px solid var(--line);
  padding: 6px 8px;
  font-family: inherit;
  font-size: 12px;
  background: #fffaf6;
}

textarea {
  min-height: 60px;
  resize: vertical;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 10px;
}

.btn {
  border-radius: 9px;
  padding: 6px 10px;
  border: 1px solid transparent;
  background: #f7efe6;
  cursor: pointer;
  font-weight: 600;
  font-size: 12px;
}

.btn.primary {
  background: var(--accent);
  color: #fff;
  box-shadow: 0 10px 24px rgba(224, 122, 63, 0.3);
}

.btn.ghost {
  background: transparent;
  border-color: var(--line);
}

.list {
  display: grid;
  gap: 8px;
}

.list-item {
  display: grid;
  grid-template-columns: 1.1fr 0.9fr auto;
  gap: 8px;
  align-items: center;
  padding: 10px 12px;
  border-radius: 11px;
  border: 1px solid var(--line);
  background: #fffaf6;
  animation: rise 0.5s ease forwards;
  animation-delay: var(--delay);
}

.list-name {
  font-weight: 600;
}

.list-sub,
.list-source {
  color: var(--muted);
  font-size: 11px;
}

.list-source {
  text-align: right;
}

.reveal-enter-active,
.reveal-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.reveal-enter-from,
.reveal-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes rise {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 1100px) {
  .content {
    grid-template-columns: 1fr;
  }

  .advanced-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 980px) {
  .app {
    grid-template-columns: 1fr;
  }

  .sidebar {
    flex-direction: row;
    overflow-x: auto;
    border-right: none;
    border-bottom: 1px solid var(--line);
  }

  .workspace {
    padding: 14px;
  }

  .topbar {
    flex-direction: column;
    align-items: flex-start;
  }

  .search {
    width: 100%;
  }
}

@media (max-width: 720px) {
  .list-item {
    grid-template-columns: 1fr;
    text-align: left;
  }

  .list-source {
    text-align: left;
  }
}

.modal-wrapper {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 9999;
}

.modal {
  width: min(720px, 100%);
  background: var(--panel);
  border-radius: 14px;
  padding: 14px;
  box-shadow: 0 22px 48px rgba(31, 22, 16, 0.3);
  max-height: 86vh;
  overflow: auto;
  position: absolute;
  left: 0;
  top: 0;
  pointer-events: auto;
}

.modal-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.modal-handle {
  cursor: move;
  user-select: none;
}
</style>
