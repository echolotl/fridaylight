<template>
  <q-dialog
    v-model="isOpen"
    persistent
    transition-show="fade"
    transition-hide="fade"
  >
    <q-card class="custom-url-download-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">Download Custom Mod</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup @click="cancel" />
      </q-card-section>

      <q-card-section>
        <p>Enter the direct download link and metadata for your mod:</p>

        <q-form @submit="onSubmit" class="q-gutter-md">
          <!-- Required Fields -->
          <q-input
            v-model="form.url"
            label="Download URL"
            outlined
            :rules="[urlValidationMessage]"
            hint="Direct link to the mod file (zip, rar, 7z)"
          >
            <template v-slot:prepend>
              <q-icon name="link" />
            </template>
          </q-input>

          <q-input
            v-model="form.name"
            label="Mod Name"
            outlined
            :rules="[(val) => !!val || 'Name is required']"
          >
            <template v-slot:prepend>
              <q-icon name="title" />
            </template>
          </q-input>

          <!-- Optional Fields in Expansion Panel -->
          <q-expansion-item
            icon="tune"
            label="Additional Options (Optional)"
            arrow-icon-color="white"
          >
            <q-card style="background-color: transparent">
              <q-card-section>
                <!-- Version Field -->
                <q-input
                  v-model="form.version"
                  label="Version"
                  class="q-mb-md"
                  outlined
                >
                  <template v-slot:prepend>
                    <q-icon name="tag" />
                  </template>
                </q-input>

                <!-- Description Field -->
                <q-input
                  v-model="form.description"
                  label="Description"
                  class="q-mb-md"
                  outlined
                  type="textarea"
                  autogrow
                >
                  <template v-slot:prepend>
                    <q-icon name="description" />
                  </template>
                </q-input>

                <!-- Banner Image Field -->
                <q-file
                  v-model="bannerFile"
                  label="Banner Image"
                  class="q-mb-md"
                  outlined
                  accept=".jpg, .jpeg, .png, .webp"
                  @update:model-value="handleBannerFileChange"
                  hint="Add a custom banner image for this mod"
                >
                  <template v-slot:prepend>
                    <q-icon name="panorama" />
                  </template>
                </q-file>
                <div v-if="bannerPreview" class="banner-preview q-mb-md">
                  <img :src="bannerPreview" alt="Banner Preview" />
                </div>

                <!-- Logo Image Field -->
                <q-file
                  v-model="logoFile"
                  label="Logo Image"
                  outlined
                  accept=".jpg, .jpeg, .png, .webp"
                  @update:model-value="handleLogoFileChange"
                  hint="Add a custom logo image for this mod"
                >
                  <template v-slot:prepend>
                    <q-icon name="image" />
                  </template>
                </q-file>
                <div v-if="logoPreview" class="logo-preview q-mb-md">
                  <img :src="logoPreview" alt="Logo Preview" />
                </div>
              </q-card-section>
            </q-card>
          </q-expansion-item>
        </q-form>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn
          flat
          label="Cancel"
          color="primary"
          v-close-popup
          @click="cancel"
        />
        <q-btn
          flat
          label="Next"
          color="primary"
          @click="onSubmit"
          :disable="!isFormValid"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";

// Interface for the form data
interface FormData {
  url: string;
  name: string;
  version?: string;
  description?: string;
  bannerData?: string;
  logoData?: string;
}

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue", "submit", "cancel"]);

// Form state
const form = ref<FormData>({
  url: "",
  name: "",
});

// File uploads
const bannerFile = ref<File | null>(null);
const logoFile = ref<File | null>(null);
const bannerPreview = ref<string | null>(null);
const logoPreview = ref<string | null>(null);

// Dialog state
const isOpen = ref(props.modelValue);

// URL validation function
const isValidUrl = (url: string): boolean => {
  try {
    // Check if it's a valid URL format
    new URL(url);

    // Check if it's an HTTP/HTTPS URL
    return url.startsWith("http://") || url.startsWith("https://");
  } catch (e) {
    return false;
  }
};

// URL validation message function
const urlValidationMessage = (url: string): string | boolean => {
  if (!url) return "URL is required";
  if (!isValidUrl(url))
    return "Please enter a valid URL (must start with http:// or https://)";
  return true;
};

// Computed property to check if form is valid
const isFormValid = computed(() => {
  return !!form.value.url && !!form.value.name && isValidUrl(form.value.url);
});

// Watch for changes in props
watch(
  () => props.modelValue,
  (val) => {
    isOpen.value = val;
  }
);

// Watch for changes in isOpen
watch(isOpen, (val) => {
  emit("update:modelValue", val);
});

// File handling methods
const handleBannerFileChange = (file: File) => {
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      bannerPreview.value = e.target?.result as string;
      form.value.bannerData = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  } else {
    bannerPreview.value = null;
    form.value.bannerData = undefined;
  }
};

const handleLogoFileChange = (file: File) => {
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      logoPreview.value = e.target?.result as string;
      form.value.logoData = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  } else {
    logoPreview.value = null;
    form.value.logoData = undefined;
  }
};

// Form submission
const onSubmit = () => {
  if (isFormValid.value) {
    emit("submit", form.value);
    resetForm();
  }
};

// Cancel form
const cancel = () => {
  emit("cancel");
  resetForm();
};

// Reset the form
const resetForm = () => {
  form.value = {
    url: "",
    name: "",
  };
  bannerFile.value = null;
  logoFile.value = null;
  bannerPreview.value = null;
  logoPreview.value = null;
};
</script>

<style scoped>
.custom-url-download-modal {
  width: 700px;
  max-width: 90vw;
  background-color: var(--theme-solid);
  border: 2px solid var(--theme-border);
  color: var(--theme-text);
}

.banner-preview,
.logo-preview {
  width: 100%;
  overflow: hidden;
  border-radius: 8px;
  margin-top: 8px;
}

.banner-preview img {
  width: 100%;
  height: auto;
  max-height: 150px;
  object-fit: cover;
}

.logo-preview img {
  height: 60px;
  width: auto;
  object-fit: contain;
}

:deep(.q-field__native),
:deep(.q-field__input) {
  color: var(--theme-text) !important;
}

:deep(.q-field__label) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__marginal) {
  color: var(--theme-text-secondary);
}

:deep(input::placeholder) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__messages) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-expansion-item__toggle-icon) {
  color: var(--theme-text);
}
</style>
