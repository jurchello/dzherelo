<script setup lang="ts">
import { computed, onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type NavItem = {
  id: string;
  label: string;
  disabled?: boolean;
};

type TabItem = {
  id: string;
  label: string;
  count: number;
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

const tabs: TabItem[] = [
  { id: "all", label: "Усі", count: 128 },
  { id: "recent", label: "Останні", count: 12 },
  { id: "needs", label: "Потребують перевірки", count: 7 },
  { id: "drafts", label: "Чернетки", count: 5 },
];

const activeNav = ref<NavItem["id"]>("persons");
const activeTab = ref<TabItem["id"]>("all");
const showAdvanced = ref(false);

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

const activeTabLabel = computed(
  () => tabs.find((tab) => tab.id === activeTab.value)?.label ?? "",
);

onMounted(() => {
  backendAvailable.value =
    typeof window !== "undefined" &&
    (window as { __TAURI_INTERNALS__?: { invoke?: unknown } })
      .__TAURI_INTERNALS__?.invoke !== undefined;
  loadPeople();
});
</script>

<template>
  <div class="app">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-mark">Дж</div>
        <div>
          <div class="brand-title">Джерело</div>
          <div class="brand-sub">genealogy workspace</div>
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
          <span v-if="item.disabled" class="nav-badge">скоро</span>
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
          <button class="btn primary">Нова особа</button>
        </div>
      </header>

      <section class="content">
        <div class="panel form-panel">
          <div class="panel-header">
            <div>
              <h2>Швидке додавання</h2>
              <p class="muted">
                Основні поля — одразу. Додаткові — згорнуті.
              </p>
            </div>
            <button class="btn ghost" @click="showAdvanced = !showAdvanced">
              {{ showAdvanced ? "Сховати деталі" : "Показати деталі" }}
            </button>
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

        <div class="panel list-panel">
          <div class="panel-header">
            <div>
              <h2>Список осіб</h2>
              <p class="muted">Вкладки для швидкого перемикання.</p>
            </div>
            <div class="tabs">
              <button
                v-for="tab in tabs"
                :key="tab.id"
                class="tab"
                :class="{ active: activeTab === tab.id }"
                @click="activeTab = tab.id"
              >
                {{ tab.label }}
                <span class="tab-count">{{ tab.count }}</span>
              </button>
            </div>
          </div>

          <div class="list-meta">
            Показано: <strong>{{ activeTabLabel }}</strong>
          </div>
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
  </div>
</template>

<style lang="scss">
:root {
  color-scheme: light;
  font-family: "Space Grotesk", "Manrope", "Segoe UI", sans-serif;
  line-height: 1.5;

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
  grid-template-columns: 260px 1fr;
  min-height: 100vh;
}

.sidebar {
  padding: 28px 20px;
  border-right: 1px solid var(--line);
  background: linear-gradient(180deg, #fff6ee 0%, #f3e8dc 100%);
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 14px;
}

.brand-mark {
  width: 48px;
  height: 48px;
  border-radius: 14px;
  background: var(--accent);
  color: #fff;
  font-weight: 700;
  font-size: 22px;
  display: grid;
  place-items: center;
  box-shadow: 0 8px 18px rgba(224, 122, 63, 0.35);
}

.brand-title {
  font-weight: 700;
  font-size: 18px;
}

.brand-sub {
  font-size: 12px;
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
  padding: 12px 14px;
  border-radius: 12px;
  border: 1px solid transparent;
  background: transparent;
  color: inherit;
  font-weight: 500;
  text-align: left;
  cursor: pointer;
  transition: transform 0.2s ease, background 0.2s ease, border 0.2s ease;
}

.nav-item:hover:not(.disabled) {
  background: rgba(255, 255, 255, 0.7);
  border-color: rgba(224, 122, 63, 0.2);
  transform: translateX(4px);
}

.nav-item.active {
  background: #fff;
  border-color: rgba(224, 122, 63, 0.4);
}

.nav-item.disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.nav-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 999px;
  background: rgba(224, 122, 63, 0.15);
  color: var(--accent-strong);
}

.sidebar-footer {
  margin-top: auto;
  font-size: 12px;
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
  padding: 28px 36px 40px;
  gap: 24px;
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
}

.topbar h1 {
  margin: 0 0 4px;
  font-size: 28px;
}

.subtitle {
  margin: 0;
  color: var(--muted);
}

.topbar-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.search {
  min-width: 280px;
  padding: 10px 14px;
  border-radius: 12px;
  border: 1px solid var(--line);
  background: #fff;
}

.content {
  display: grid;
  grid-template-columns: minmax(300px, 380px) 1fr;
  gap: 22px;
}

.panel {
  background: var(--panel);
  border-radius: 18px;
  padding: 22px;
  box-shadow: var(--shadow);
  animation: fadeIn 0.6s ease forwards;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
  margin-bottom: 18px;
}

.muted {
  color: var(--muted);
}

.form-grid {
  display: grid;
  gap: 12px;
}

.advanced-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-top: 14px;
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
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--muted);
}

input,
select,
textarea {
  border-radius: 12px;
  border: 1px solid var(--line);
  padding: 10px 12px;
  font-family: inherit;
  font-size: 14px;
  background: #fffaf6;
}

textarea {
  min-height: 84px;
  resize: vertical;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 18px;
}

.btn {
  border-radius: 12px;
  padding: 10px 16px;
  border: 1px solid transparent;
  background: #f7efe6;
  cursor: pointer;
  font-weight: 600;
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

.tabs {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tab {
  border-radius: 999px;
  padding: 6px 12px;
  border: 1px solid transparent;
  background: #f5ede5;
  font-size: 13px;
  cursor: pointer;
}

.tab.active {
  border-color: var(--accent);
  background: #fff;
  color: var(--accent-strong);
}

.tab-count {
  margin-left: 6px;
  font-weight: 700;
}

.list-meta {
  margin-bottom: 12px;
  color: var(--muted);
  font-size: 13px;
}

.list {
  display: grid;
  gap: 12px;
}

.list-item {
  display: grid;
  grid-template-columns: 1.2fr 1fr auto;
  gap: 12px;
  align-items: center;
  padding: 14px 16px;
  border-radius: 14px;
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
  font-size: 13px;
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

@media (max-width: 860px) {
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
    padding: 20px;
  }

  .topbar {
    flex-direction: column;
    align-items: flex-start;
  }

  .search {
    width: 100%;
  }
}
</style>
