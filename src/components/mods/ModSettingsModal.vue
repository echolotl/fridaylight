<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Mod Settings</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-card-section v-if="mod">
        <q-input 
          v-model="form.name" 
          label="Mod Name" 
          outlined 
          class="q-mb-md"
        />
        
        <q-input 
          v-model="form.version" 
          label="Version" 
          outlined 
          class="q-mb-md"
          placeholder="e.g. 1.0.0"
        />
        
        <q-select
          v-model="form.engine_type"
          :options="engineTypes"
          label="Engine Type"
          outlined
          class="q-mb-md"
          emit-value
          map-options
        />
        
        <q-input 
          v-model="form.path" 
          label="Folder Location" 
          outlined 
          readonly
          class="q-mb-md"
        >
          <template v-slot:append>
            <q-btn round flat icon="folder" @click="$emit('change-folder')" />
          </template>
        </q-input>
        
        <q-input 
          v-model="form.executable_path" 
          label="Executable Path" 
          outlined 
          readonly
          class="q-mb-md"
        >
          <template v-slot:append>
            <q-btn round flat icon="file_open" @click="$emit('select-executable')" />
          </template>
        </q-input>
        
        <div class="banner-upload q-mb-md">
          <div class="text-subtitle2 q-mb-sm">Banner Image</div>
          <div class="banner-preview" v-if="bannerPreview || form.banner_data">
            <img :src="bannerPreview || form.banner_data" alt="Banner Preview" />
          </div>
          <div class="banner-placeholder" v-else>
            <q-icon name="image" size="48px" />
            <div>No banner image</div>
          </div>
          <q-file
            v-model="bannerFile"
            label="Upload Banner Image"
            outlined
            accept=".jpg, .jpeg, .png"
            @update:model-value="handleBannerFileChange"
            class="q-mt-sm"
          >
            <template v-slot:prepend>
              <q-icon name="cloud_upload" />
            </template>
          </q-file>
        </div>
        
        <div class="logo-upload q-mb-md">
          <div class="text-subtitle2 q-mb-sm">Logo Image (Replaces Title)</div>
          <div class="logo-preview" v-if="logoPreview || form.logo_data">
            <img :src="(logoPreview || form.logo_data) || ''" alt="Logo Preview" />
          </div>
          <div class="logo-placeholder" v-else>
            <q-icon name="image" size="48px" />
            <div>No logo image</div>
          </div>
          <q-file
            v-model="logoFile"
            label="Upload Logo Image"
            outlined
            accept=".jpg, .jpeg, .png"
            @update:model-value="handleLogoFileChange"
            class="q-mt-sm"
          >
            <template v-slot:prepend>
              <q-icon name="cloud_upload" />
            </template>
          </q-file>
          <q-btn 
            v-if="logoPreview || form.logo_data" 
            flat 
            color="negative" 
            label="Remove Logo" 
            class="q-mt-sm"
            @click="removeLogo"
          />
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" v-close-popup @click="cancel" />
        <q-btn flat label="Save" color="primary" @click="save" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';

interface Mod {
  id: string;
  name: string;
  path: string;
  executable_path?: string;
  icon_data?: string;
  banner_data?: string;
  logo_data?: string | null;
  version?: string;
  engine_type?: string;
}

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  mod: {
    type: Object as () => Mod | null,
    default: null
  }
});

const emit = defineEmits(['update:modelValue', 'save', 'change-folder', 'select-executable']);

const form = ref<Mod>({
  id: '',
  name: '',
  path: '',
  executable_path: '',
  icon_data: '',
  banner_data: '',
  logo_data: '',
  version: '',
  engine_type: ''
});

const bannerFile = ref<File | null>(null);
const bannerPreview = ref<string | null>(null);
const logoFile = ref<File | null>(null);
const logoPreview = ref<string | null>(null);

const engineTypes = [
  { label: 'Vanilla', value: 'vanilla' },
  { label: 'Psych Engine', value: 'psych' },
  { label: 'Codename Engine', value: 'codename' },
  { label: 'FPS Plus', value: 'fps-plus' },
  { label: 'Kade Engine', value: 'kade' },
  { label: 'Pre-VSlice', value: 'pre-vslice' },
  { label: 'Other', value: 'other' }
];

const showModal = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});

// Reset form when modal is opened
watch(() => props.modelValue, (newVal) => {
  if (newVal && props.mod) {
    // Clone the mod object to form
    form.value = JSON.parse(JSON.stringify(props.mod));
    bannerPreview.value = null;
    logoPreview.value = null;
    bannerFile.value = null;
    logoFile.value = null;
  }
});

const handleBannerFileChange = (file: File) => {
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      bannerPreview.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  } else {
    bannerPreview.value = null;
  }
};

const handleLogoFileChange = (file: File) => {
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      logoPreview.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  } else {
    logoPreview.value = null;
  }
};

const removeLogo = () => {
  form.value.logo_data = null;
  logoPreview.value = null;
  logoFile.value = null;
};

const save = () => {
  const updatedMod = { ...form.value };
  
  // If we have a banner preview, use it
  if (bannerPreview.value) {
    updatedMod.banner_data = bannerPreview.value;
  }
  
  // If we have a logo preview, use it
  if (logoPreview.value) {
    updatedMod.logo_data = logoPreview.value;
  }
  
  emit('save', updatedMod);
};

const cancel = () => {
  bannerPreview.value = null;
  logoPreview.value = null;
  bannerFile.value = null;
  logoFile.value = null;
};
</script>

<style scoped>
.settings-modal {
  width: 500px;
  max-width: 90vw;
  background-color: var(--theme-card);
  color: var(--theme-text);
}

.banner-upload, .logo-upload {
  display: flex;
  flex-direction: column;
}

.banner-preview img, .logo-preview img {
  width: 100%;
  height: auto;
  max-height: 150px;
  object-fit: contain;
  border-radius: 4px;
}

.banner-placeholder, .logo-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--theme-text-secondary);
  border: 1px dashed var(--theme-border);
  border-radius: 4px;
  padding: 16px;
}
</style>