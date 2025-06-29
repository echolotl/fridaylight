<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Loading overlay shown during initialization -->
    <div v-if="isInitializing" class="loading-overlay">
      <div class="loading-container">
        <svg
          id="Layer_2"
          class="logo"
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          viewBox="0 0 633.82 196.75"
        >
          <defs>
            <linearGradient
              id="linear-gradient"
              x1="154.74"
              y1="1003.28"
              x2="154.74"
              y2="906.88"
              gradientTransform="translate(0 -852)"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0" stop-color="#fc0" />
              <stop offset="1" stop-color="#f7931e" />
            </linearGradient>
            <linearGradient
              id="linear-gradient-2"
              x1="355.89"
              y1="1014.93"
              x2="355.89"
              y2="886.93"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-3"
              x1="394.47"
              y1="1001.08"
              x2="394.47"
              y2="891.18"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-4"
              x1="389.09"
              y1="977.48"
              x2="389.09"
              y2="895.38"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-5"
              x1="450.8"
              y1="1013.28"
              x2="450.8"
              y2="876.07"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-6"
              x1="518.48"
              y1="1011.98"
              x2="518.48"
              y2="879.48"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-7"
              x1="567.09"
              y1="1011.98"
              x2="567.09"
              y2="871.92"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-8"
              x1="148.24"
              y1="1003.68"
              x2="148.24"
              y2="908.08"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-9"
              x1="121.14"
              y1="1015.58"
              x2="121.14"
              y2="911.28"
              xlink:href="#linear-gradient"
            />
            <linearGradient
              id="linear-gradient-10"
              x1="155.44"
              y1="959.48"
              x2="155.44"
              y2="906.98"
              xlink:href="#linear-gradient"
            />
          </defs>
          <g id="Layer_1-2" data-name="Layer_1">
            <g id="Layer_2-2" data-name="Layer_2">
              <g id="Layer_1-2">
                <g>
                  <path
                    class="cls-4"
                    d="M226.54,23.93c-1.34-2.72.76-5.89,3.1-7.82,1.52-1.25,3.82-2.29,5.31-1.01l.47.06c-.18.39-.36.78-.54,1.17.17-.61,1.11-.67,1.56-.23.45.45.53,1.14.53,1.77.03,2.79-.91,5.78-3.19,7.4-2.28,1.61-6.02,1.17-7.25-1.34ZM607.46,23.69c1.14,1.32,2.98,1.84,4.72,1.87,1.22.02,2.47-.17,3.54-.77,1.86-1.03,2.93-3.31,2.55-5.4-.89-4.87-7.16-5.63-10.3-2.47-1.75,1.76-2.12,4.88-.5,6.76ZM627.95,126.31c-4.93,1.07-7.9,5.97-10.29,10.41-.32.59-.65,1.24-.55,1.91.13.88.95,1.48,1.77,1.83,3.31,1.44,7.24.36,10.25-1.62,2.42-1.59,4.59-4.05,4.56-6.94,0,0,.59-3.61-.81-5.6-1.49-2.11-4.93,0-4.93,0ZM630.76,154.5c-4.56,5.48-15.56,9.94-21.17,8.58-.62-.15-1.16-.36-1.67-.6-.09,3.58-1.55,6.87-4.25,9.38-3.65,3.4-9.11,4.92-14.23,3.95l-.09-.02-19.07.34c-6.02,4.92-14.51,8.73-21.89,10.19-12.02,2.37-10.64-2-8.66-8.83.08-.27.17-.54.25-.82l-122.81,2.19c-6.68,1.02-13.43,1.63-20.22,1.96-1.92,2.13-3.78,4.36-6.23,5.76-3,1.72-6.6,2.05-10.05,1.8-3.03-.22-6.09-.89-8.65-2.52-1.42-.91-4.17-3.06-6.48-5.23-8.44-.4-16.85-1-25.17-1.6-7.63-.55-15.51-1.11-23.19-1.5-2.91,2.07-5.18,4.07-7.22,5.85-4.51,3.96-8.41,7.38-15.44,8.24l-.24.03h-.24c-3.15,0-7.42-.76-14.02-5.42-2.48-.84-4.67-2.3-6.93-3.56-1.26-.7-2.74-1.37-4.17-1.03-2.91.69-2.87,4.36-4.22,6.78-1.7,3.05-6.71,4.07-9.71,1.98-3-2.08-3.17-6.7-.32-8.95,2.86-2.27,7.93-2.32,9.37-5.5.44-.98.42-2.07.39-3.13-.04-1.16-.15-2.47-1.16-3.18-.92-.65-2.23-.55-3.39-.39-2.36.32-4.76.75-7.12.41-3.25-.47-4.64-1.75-4.86-3.41-3.16-.08-6.42-.45-9.7-1.26-6.33,15.63-13.69,23.5-22.43,24.02-8.99.56-17.71-6.93-25.95-22.21h-16.85c-1.19.31-2.49.68-3.88,1.07-11.85,3.32-28.08,7.87-36.56.53-3.04-2.63-4.67-6.41-4.85-11.25-7.53.45-14.49-.18-19.47-7.21,0,.11,0,.24,0,.36,1.57,2.12,2.5,4.62,2.39,7.31-.18,4.32-2.94,8.19-6.42,10.76-2.15,1.59-4.56,2.75-7.05,3.73,1.06,2.33,3.16,4.11,5.18,5.73,2.29,1.83,4.7,3.79,5.69,6.54,1.49,4.14-1.29,9.28-5.57,10.3s-9.08-2.32-9.6-6.7c-.27-2.27.45-4.54.45-6.83,0-2.29-1.13-4.92-3.37-5.33-2.63-.49-4.59,2.19-6.42,4.14-3.05,3.27-7.52,5.18-11.99,5.12-3.78-.05-7.44-1.42-11.21-1.59-6.64-.3-13.07,3.1-19.69,2.59-1.13-.09-2.3-.38-3.39-.83-6.77.65-12.28.94-15.42.94-18.8,0-19.97-16.03-20.75-26.63-.21-2.9-.41-5.65-.86-7.98l-.05-.33c-1.31-12.35,10.26-16.27,18.22-18.47-.1-5.58-1.18-13.68-2.39-22.84-1.2-9.05-2.47-18.72-3.16-28.1-.13-.08-.26-.16-.38-.25-1.37-1.04-2.28-2.56-3.01-4.12-2.99-6.36-3.03-14.56,1.59-19.86.7-.8,1.54-1.64,1.58-2.7.03-.76-.37-1.46-.7-2.15-1.83-3.83-1.7-8.53.33-12.26,1.8-3.29,5.07-6.21,4.79-9.95-.31-4.14-4.76-6.55-8.71-7.84-1.1-.36-2.27-.73-3.07-1.57-1.72-1.79-1-4.8.38-6.86,2.08-3.11,5.43-5.33,9.1-6.04,2.72-.52,6.04.07,7.29,2.54,2.2,4.35-3.85,9.91-1.25,14.03,1.76,2.8,5.86,2.32,9.16,2.11,3.04-.19,6.12.14,9.05.99.71.2,1.42.44,2.16.41,1.31-.06,2.42-.95,3.47-1.74,3.4-2.55,7.36-4.46,11.59-4.93,2.68-.3,7.09.04,10.12,1.59,24.17-4.16,53.49-6.11,65.51,5.5,3.01,2.9,6.04,7.77,5.45,15.39,8.08-1.33,16.21-2.47,24.12-3.58,3-.42,5.98-.84,8.92-1.26l.52-.04c4.67-.07,9.29,2.24,12.65,6.32,1.21,1.47,2.17,3.07,2.87,4.74,6.06-13.06,15.55-14.5,31.78-14.15l.41.03c.94.12,3.78.05,6.53-.02,1.11-.03,2.15-.05,3.15-.06,1.93-3.73,6.4-7.7,9.12-9.06,2.72-1.36,5.7-2.11,8.53-3.25,1.03-.41,2.12-.95,2.58-1.94.6-1.29-.15-2.84-1.23-3.79s-2.47-1.5-3.66-2.33c-3.78-2.63-4.97-8.2-2.58-12.08,1.19-1.92,3.23-3.43,5.53-3.61,2.3-.18,4.75,1.19,5.38,3.35,1,3.39-2.44,6.69-2.1,10.2.22,2.26,2.05,4.16,4.23,4.98.86.32,2.08.65,3.37.9.65-.07,1.32-.1,2.01-.1l.38.02c1.08.1,2.16.21,3.24.32.32-.04.74.01,1.21.13,9.71,1.05,19.5,2.66,30.39,6.99,14.11-2.36,38.97-6.23,53.16,2.86,7.25,4.69,10.27,7.87,10.97,21.12.54,2.79,1.21,4.98,2.31,5.14,2.21.32,4.36-1.99,6.77-4.5.76-.98,1.66-1.86,2.67-2.62,1.97-1.74,4.15-3.12,6.69-3.12l.13.16c6.07-1.31,14.44-1.31,17.98-1.19,2.08-1.44,5.74-4.32,9.59-7.35,20.38-16.05,31.28-23.67,37.68-23.16,6.03.09,11.64,4.05,16.75,8.49,2.05-1.74,4.64-3.16,7.03-2.13,1.14.49,1.98,1.48,3.01,2.18,1.03.7,2.49,1.08,3.46.3.91-.73.93-2.09.78-3.25-.37-2.88-1.3-5.72-1.11-8.61s1.95-6.01,4.82-6.45c1.58-.24,3.18.37,4.48,1.29,1.63,1.14,2.96,2.89,3.13,4.87.2,2.28-1.11,4.39-2.49,6.21-1.38,1.82-2.94,3.65-3.38,5.89-.2,1.03-.05,2.16.39,3.12,5.42-1.69,12.71-3.71,22.51-6.33l2.95-.79c1.86-2.98,4.8-5.14,8.78-6.43,10.37-3.37,27.21-.39,44.65,4.97,4.61.17,12.05.55,19.61,3.28,10.09,3.64,6.29,5.11,10.66,5.17,4.7.05,7.02-6.85,6.33-11.5-.25-1.69-.9-3.38-.6-5.06.29-1.63,1.45-2.98,2.77-3.97,2.35-1.76,5.54-2.6,8.31-1.63s4.83,4.04,4.14,6.9c-.92,3.84-5.57,5.29-8.16,8.27-1.64,1.88-2.43,4.53-1.89,6.96s2.49,4.56,4.92,5.11c1.43.33,2.95.13,4.38.46,4.56,1.07,6.2,6.64,9.63,9.82,1.89,1.75,4.38,2.8,6.12,4.71,1.03,1.13,1.69,2.43,2.11,3.84,1.41,2.27,2.31,4.9,2.64,7.84.44,3.9-.12,8-1.33,11.93-2.01,7.79-6.24,15.11-10.54,19.1-7.62,7.08-14.2,6.86-20.04,6.25,1.84,8.58,3.51,17.36,5.03,26.29,7.4,1.47,12.23,5.38,11.23,11.55-.44,2.69-1.93,5.68-2.25,8.34-.31,2.66.87,5.08,4.55,5.19,2.58.08,5.87-.98,8.75-1.33,7.5-.91,10.68,4.03,6.12,9.51ZM603.87,160.89c-2.6-23.47-6.39-47.15-11.59-70.32.89.05,1.78.08,2.66.11,6.67.67,12.29,1.99,19.72-4.91,3.89-3.62,7.7-10.46,9.45-17.51,2-6.6,1.93-13.55-2.33-18.2-.79-.93-1.72-1.75-2.83-2.45l-.05.07c-21.75-12.81-101.67-47.61-111.36-25.35-17.78,4.79-37.37,9.79-39.47,13.09-6.79-4.3-16.59-17.28-25.78-17.28-8.19-1-39.27,26.57-45.96,30.57-7.19-.9-20.88.7-22.28,2.5-2.2,1.2-3.9,3.2-4.7,5.59-6.89,22.57-10.19,31.56.3,41.25-1.2,3.3-4.2,16.18-5.8,21.38h-5.5c-.9-19.28-3.5-42.65-3.5-61.43s-2.5-20.58-9.29-24.97c-12.79-8.19-35.97-4.69-51.46-2.1-11.89-4.89-22.18-6.39-34.77-7.59-10.99,0-14.49,11.09-15.19,18.38-3.1-2.9-22.18-1.1-26.18-1.6-18.28-.4-25.78,1-30.97,20.08-1.2-.5-2.4-1-3.6-1.4,1.5-7.19-5.1-15.68-12.59-15.58-12.49,1.8-25.68,3.5-38.57,5.79,11.99-42.05-82.03-19.78-101.51-13.48-27.38,8.19-7.19,79.11-8.49,100.29-7.59,2.1-19.28,4.59-18.18,14.98,2.3,11.99-1.5,31.37,17.68,31.37s85.73-8.19,82.53-25.57c.3-5.89-1.8-12.09,4.2-14.88,4.5,12.49,11.89,12.09,23.38,11.09-3.2,24.57,27.08,13.49,40.57,9.99h19.78c12.99,25.37,29.77,34.66,43.56-2.7,10.49,3.7,21.38,2.2,31.77-.8l3.3,11.79c.5,1.7,1.4,3.3,2.7,4.49,12.59,12.09,18.28,12.09,20.48,12.09,8.99-1.1,11.69-7.19,21.98-14.18,33.27,1.6,67.84,6.49,100.81,1.4l173.25-3.1.1.1c6.89,1.3,14.59-3.3,13.69-10.99ZM531.44,178.89c-5.62-2.89-21.84,14.56-24.5,17.86,1.8-2.24,29.84-15.11,24.5-17.86Z"
                  />
                  <path
                    d="M590.18,171.88c6.89,1.3,14.59-3.3,13.69-10.99-2.6-23.47-6.39-47.15-11.59-70.32,7.89.4,15.19.5,23.38-7.69,8.19-8.19,15.39-27.57,3.3-35.16-21.68-12.79-101.71-47.65-111.41-25.37-17.78,4.79-37.37,9.79-39.47,13.09-6.79-4.3-16.59-17.28-25.78-17.28-8.19-1-39.27,26.57-45.96,30.57-7.19-.9-20.88.7-22.28,2.5-2.2,1.2-3.9,3.2-4.7,5.59-6.89,22.57-10.19,31.56.3,41.25-1.2,3.3-4.2,16.18-5.8,21.38h-5.5c-.9-19.28-3.5-42.65-3.5-61.43s-2.5-20.58-9.29-24.97c-12.79-8.19-35.97-4.69-51.46-2.1-11.89-4.89-22.18-6.39-34.77-7.59-10.99,0-14.49,11.09-15.19,18.38-3.1-2.9-22.18-1.1-26.18-1.6-18.28-.4-25.78,1-30.97,20.08-1.2-.5-2.4-1-3.6-1.4,1.5-7.19-5.1-15.68-12.59-15.58-12.49,1.8-25.68,3.5-38.57,5.79,11.99-42.05-82.03-19.78-101.51-13.48-27.38,8.19-7.19,79.11-8.49,100.29-7.59,2.1-19.28,4.59-18.18,14.98,2.3,11.99-1.5,31.37,17.68,31.37s85.73-8.19,82.53-25.57c.3-5.89-1.8-12.09,4.2-14.88,4.5,12.49,11.89,12.09,23.38,11.09-3.2,24.57,27.08,13.48,40.57,9.99h19.78c12.99,25.37,29.77,34.66,43.56-2.7,10.49,3.7,21.38,2.2,31.77-.8l3.3,11.79c.5,1.7,1.4,3.3,2.7,4.49,12.59,12.09,18.28,12.09,20.48,12.09,8.99-1.1,11.69-7.19,21.98-14.18,33.27,1.6,67.84,6.49,100.81,1.4l173.25-3.1.1.1ZM592.28,90.47c7.89.4,13.99,3.1,22.38-4.69s16.39-30.57,4.3-38.16"
                  />
                </g>
              </g>
              <g id="Layer_1-2-2">
                <path
                  class="cls-3"
                  d="M26.54,65.48c-.5-8.4-3.2-22,4.7-24.8,17.7-6.3,44.1-8.6,62.7-10.9,6.8-.8,29.3.8,30.1,7.6l-10.8,39-29.1,2.8c-4.9-15.1-12.4-5.2-22.3-.3-2.2.5.5,9.7.3,11l34.2-7.5c6.2,38.5-3,21.4-30.9,34.8l4.6,45.5c0,.2-.4,0-.2,0-16.1,2-34.3,5.1-50.1,8.8-.7.2-1.6.3-2-.4-.1-.2-.2-.4-.2-.7-1.5-8.1-2.6-16.2-3.2-24.4,0-.8.5-1.6,1.3-1.7l17.6-5.3-6.5-73.4h0l-.2-.1Z"
                />
                <path
                  class="cls-3"
                  d="M120.34,138.78c8.2,1,13-.8,18.6-.3-.8-10.5-2.2-22.6-8.7-28-5-4.2-11.4-2.9-17.2-1.2-7.9,2.3-16.2,5.4-21.5,13.8-1-3,1-7.6-1.1-9.2-.8-1.7-17.7,6.3-18.9,6.4,1.8,13.7,3.5,27.4,5.3,41.1,5.7-2.7,11.6-4.5,17.6-5.5-.3-7.3-.4-15.2,2.7-21.1,2.9-5.6,14.1-10.5,18.1-6.3l5.2,10.3h-.1Z"
                />
                <path
                  class="cls-12"
                  d="M141.94,154.18c1.4,1,6.8-.5,8.3-.7,5.8-.8,11.6-1.9,17.3-3.2-3.3-13.8-7.2-27.4-11.6-40.9l-13.5,1.5-.5,43.3h0Z"
                />
                <path
                  class="cls-3"
                  d="M207.84,81.88c-9-9.4-17.7-14.9-29.7-19-6.7,9.6-18.3,24.3-18.3,37,1.7,13.8,11.4,48.9,11.4,48.9h29c7.3-4.3,14.3-16.9,15.8-27.9-.2-7.9-5.9-32.9-8.2-39h0ZM190.04,122.98c-1.2-7.6-2.3-20.8-1.6-30.4,5.2,4.8,9.6,6.8,13.1,17.9-4.5,7-7.1,12.3-11.4,12.6h0l-.1-.1Z"
                />
                <path
                  class="cls-3"
                  d="M218.24,45.98c-4.1,0-8.3,0-12.4.5-1,.1-2.1.3-3,.8-1.1.6-1.9,1.5-2.6,2.5l-6.3,17.2c6.2,3.1,12.9,7.3,16.5,13.3,4.5,9.4,5.3,20.2,7.7,30.2.8,4.4,1.6,8.9,1.1,13.3-.5,4.1-2.1,8-3.9,11.7-1.1,2.4-3.2,7.2-6.3,10.3s-6.1,5.6-7.5,7.8c0,0,7.2,15.3,13.6,18,11.1-6.8,20.8-48.6,20.8-48.6,2,4.5,1.2,18,3.3,22.5.2.5.5,1.1.9,1.5.6.5,1.3.6,2.1.8,9,1.4,18.5.2,26.9-3.4,0,0-9.2-83-29.8-98.3h-20.8l-.3-.1ZM244.84,88.28c-4.5,18.5-8.3,9.9-13.7-1.7-2.4-5.1,14.9-1.9,13.7,1.7Z"
                />
                <path
                  class="cls-3"
                  d="M296.14,138.68l30.5-6.8c-1.4,6.4,0,13.3-3.1,19.1-6.9,5.7-19.1,15.4-28.8,22.8-3.6.7-14-9.3-14-9.3l-8.3-29.9-6.6-35.4-12.1-56.3,1.5-10.5c0-.7.2-1.4.7-1.8.6-.5,1.4-.5,2.2-.5,9.8.7,19.5,2.1,29,4.3.9.2,1.8.4,2.4,1,1.4,1.2,1.3,3.4,1.1,5.3-1.7,14.1-1.8,28.4-.3,42.6,7.2-9.7,16.7-17.8,27.5-23.4,3.4,8.8,6.9,17.6,10.3,26.4,1.1,2,.3,4-1.5,5.4-9.4,8.5-19.6,16.4-27.4,26.4-1.2,1.6-2.4,3.2-2.8,5.2l-.2,15.2h0l-.1.2Z"
                />
                <path
                  class="cls-1"
                  d="M329.64,78.48c1.7,4.1,3.3,8.8,1.5,12.8l-14.8,16.8-.4,17.7h0c-.3,2.2,1.8,4.1,3.9,3.5l9.9-.2s-.7,16.7-3,21.7c-1.4,3-7.7,9.1-7.7,9.1l7.4.2c28.3,2.7,56.7,3.5,85.1,2.3,1.8,0,3.1-1.6,3-3.3-.6-10.3-1.3-20.6-2.2-30.9,0-.9-.2-1.8-.8-2.4-.7-.7-1.8-.7-2.8-.7l-56.3.4c-1.7,0-3.1-1.3-3.2-2.9-1.2-18.3-.8-44.8-2-62.8-.4-5.7-.9-13.8-3.1-19.2-3.2-8-33.7-5.2-43.8-5.1-1.8,0-3.2,1.5-3.1,3.3.3,8.2.2,16.5,0,24.7,0,2.5,2.6,4.1,4.8,2.8l13.2-10,4.6-2.8,9.9,25.2-.1-.2Z"
                />
                <path
                  class="cls-6"
                  d="M409.44,90.48c8.8.2,9.5-24.5,9.5-28.7-7.1-7.1-36.8-9.3-42.8-6-13.5,40.9-5.1,30.2,28.2,35.7l5-1h.1Z"
                />
                <path
                  class="cls-5"
                  d="M371.94,122.68h34.3c-1.4-9-2.9-18-4.3-27-.6-3.8-12.8-1.4-16.3-2.1-4.6,0-4.5,0-6.2,3.8-1.1,2.4-6.1,25.3-7.5,25.3Z"
                />
                <path
                  class="cls-7"
                  d="M415.64,91.38c-3.2,2.8-3.9,4.2-8.2,4.3.8,6.3,1.6,12.7,2.5,19,.3,2.4.8,5,2.6,6.5.8.7,1.9,1.1,2.6,1.8,1,1,1.1,2.5,1.3,3.9,1.1,11.4,2.2,22.8,3.3,34.2,19.9-.5,36.8-1,56.7-1.5,3.3-13.6,6.7-27.3,10-40.9-6.2.6-12.3,1.1-18.5,1.6,0,4.5-.2,9-.8,13.5,2.2,1.7,4.8,2.9,7.4,3.9-6.3,1.9-17.7,5.3-21.3,0,4.3-15.5,8.2-25.6,12.5-39.8,1.1-3.5,1-5.5,3.9-12.3,2.6,2.8.1,21.1.1,28.7,8.6,1.6,11.2,1,18.8,1.6,3-12.4,6.1-30.9,9.1-43.3.8-2.6,0-4.2-2.1-5.8-16.5-13.7-33.1-27.5-49.6-41.2-.9-.8-1.9-1.5-3-1.7-1-.1-1.9.3-2.9.6l-36.4,27,17.2,5.6,1.7,2.4c-.8,6.7-.8,15.3-2.1,21.8l-4.9,10h0l.1.1Z"
                />
                <path
                  class="cls-11"
                  d="M545.94,49.88c1.4.2,1.8.8,1.5,2.2-2.5,24.4-16.4,63.2,2.6,79.9,5.3,4.7,14.5,3.1,14.5,4.7-.1,4.7,0,23.3,0,23.3l-56.6-1.4c-1-4.3,1.9-35.9,2.1-40.3-2.1,4.6-7.7,36.5-9.9,41.1-6.5.1-15,.2-21.5.3,9.1-29,17.6-58.6,22.2-88.6-.3-1.2.7-2.2-.3-3-8.3-7.1-16.6-14.2-25-21.2-6-5.1-2.4-7.2,2.8-9.9,1.4-.8,33.8-9.4,33.7-9.5,1.6,16,2,43.8,2,64.2,2.9-6.7,7-29.9,10.7-36.2.4-.7.9-1.4,1.6-1.8l19.5-3.7h0l.1-.1Z"
                />
                <path
                  class="cls-9"
                  d="M553.94,130.68c.7.6,13.8,3.7,13.9,4.6.8,8.2-.4,16.5.4,24.7,8.3-1.7,17.7-2.4,26.1-2.1-3-27.7-7.8-55.3-14.4-82.4l28.3,6.8c6.2-9.4,10.1-19.8,10.1-29.2-28.3-16.9-62.1-26.4-94.8-33-3.9-.9-7.6,2-7.7,6.1,0,5.8.8,14,1,19.8.3,5.8,1.3,6.7,7,5.7,5.1-.9,13.7-3.8,18.8-4.8,2.8-.5,6.6-.6,7.7,2.1l-3.6,28.5v8c1.5,14.7,3,21.4,4.5,36.2l2.7,9h0Z"
                />
                <g>
                  <path
                    class="cls-8"
                    d="M173.94,52.08l-2.1-3c-1.2-.3-.6-.1-1.8,0-12.7,1.9-33.3,4.4-45.7,7.5l-1.8,5.7,51.4-10.3h0v.1Z"
                  />
                  <g>
                    <path
                      class="cls-10"
                      d="M131.24,107.18l2.2-41.2-13.2,3.5-11.4,37.3c7.7-1.7,15-2.5,22.3.4h.1Z"
                    />
                    <path
                      class="cls-2"
                      d="M174.94,54.98l-40.9,10.9,4.8,41.6c5.8-.8,11.5-1.6,17.3-2.4-1.2-13.2,4.9-24.3,10.8-35.7l9.9-9.4s-1.9-5-1.9-5Z"
                    />
                  </g>
                </g>
              </g>
            </g>
          </g>
        </svg>
        <q-linear-progress
          :value="initProgress"
          size="md"
          rounded
          class="q-mt-md"
          color="primary"
        />
        <div class="text-caption q-mt-sm phantom-font">
          {{ initStatusText }}
        </div>
      </div>
    </div>
    <div v-if="!isInitializing">
      <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize" />

      <!-- Missing mod path dialog -->
      <MessageDialog
        v-model="showModPathMissingDialog"
        title="Missing Mod"
        message="The following mod folder no longer exists:"
        icon="warning"
        icon-color="negative"
        confirm-label="Remove From List"
        confirm-color="negative"
        @confirm="removeMissingMod"
      >
        <div class="text-caption">{{ missingModPath }}</div>
        <p class="text-body2 q-mt-sm">
          Would you like to remove this mod from your library?
        </p>
      </MessageDialog>

      <!-- Global Context Menu -->
      <ContextMenu
        :visible="showContextMenu"
        :position="contextMenuPosition"
        :options="contextMenuOptions"
        @close="closeContextMenu"
      />
    </div>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { onOpenUrl } from '@tauri-apps/plugin-deep-link'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { openUrl } from '@tauri-apps/plugin-opener'
import Sidebar from './components/layout/Sidebar.vue'
import MessageDialog from './components/modals/MessageDialog.vue'
import ContextMenu from './components/common/ContextMenu.vue'
import { DatabaseService } from '@services/dbService'
import { StoreService, DEFAULT_SETTINGS } from '@services/storeService'
import { themeService } from '@services/themeService'
import { AppSettings } from './types'
import { notificationService } from '@services/notificationService'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// Define window.db
declare global {
  interface Window {
    db: any
  }
}

// Initialization state variables
const isInitializing = ref(true)
const initProgress = ref(0)
const initStatusText = ref(t('app.notifications.initialization.starting_up'))

const sidebarOpen = ref(true)
const sidebarWidth = ref(250)

const handleSidebarResize = (width: number) => {
  sidebarWidth.value = width
}

// State for missing mod path dialog
const showModPathMissingDialog = ref(false)
const missingModPath = ref('')

// Function to remove a mod when it's missing
const removeMissingMod = async () => {
  if (!missingModPath.value) return

  console.log('Removing missing mod with path:', missingModPath.value)

  try {
    // Find mod ID based on path
    const dbService = DatabaseService.getInstance()
    const mod = await dbService.getModByPath(missingModPath.value)

    if (mod) {
      // Delete the mod from the database
      await dbService.deleteMod(mod.id)

      // Show success notification
      notificationService.success({
        message: 'Mod removed successfully!',
      })

      // Refresh the mods list by dispatching an event
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)
    } else {
      console.warn('Could not find mod with path:', missingModPath.value)
    }
  } catch (error) {
    console.error('Error removing missing mod:', error)
    notificationService.error({
      message: 'Failed to remove mod',
      caption: String(error),
    })
  } finally {
    // Reset state
    showModPathMissingDialog.value = false
    missingModPath.value = ''
  }
}

// State for context menu
const showContextMenu = ref(false)
const contextMenuPosition = ref({ x: 0, y: 0 })
const contextMenuOptions = ref<any[]>([])

// Function to close the context menu
const closeContextMenu = () => {
  showContextMenu.value = false
  contextMenuOptions.value = []
}

// Global event listener for custom context menu
onMounted(() => {
  // Prevent default context menu
  document.addEventListener('contextmenu', event => {
    event.preventDefault()
  })

  // Listen for custom context menu events
  document.addEventListener('show-context-menu', ((event: CustomEvent) => {
    // Get position and options from the event
    const { position, options } = event.detail

    // Update context menu state
    contextMenuPosition.value = position
    contextMenuOptions.value = options
    showContextMenu.value = true
  }) as EventListener)
})

// Cleanup event listeners on unmount
onUnmounted(() => {
  document.removeEventListener('contextmenu', event => {
    event.preventDefault()
  })

  document.removeEventListener('show-context-menu', (() => {}) as EventListener)

  // Clean up global function
  delete (window as typeof window & { handleUrlClick?: (url: string) => void })
    .handleUrlClick
})

// Initialize application settings
const appSettings = reactive<AppSettings>({ ...DEFAULT_SETTINGS })

// Provide app settings to all components
provide('appSettings', appSettings)

// Load app settings from database and update the reactive object
const loadAppSettings = async () => {
  try {
    const storeService = StoreService.getInstance()

    // Load each setting individually
    const accentColor = await storeService.getSetting('accentColor')
    if (accentColor) appSettings.accentColor = accentColor

    const installLocation = await storeService.getSetting('installLocation')
    if (installLocation) appSettings.installLocation = installLocation

    const theme = await storeService.getSetting('theme')
    if (theme) appSettings.theme = theme
    const useSystemTheme = await storeService.getSetting('useSystemTheme')

    if (useSystemTheme !== undefined)
      appSettings.useSystemTheme = useSystemTheme === true

    const validateFnfMods = await storeService.getSetting('validateFnfMods')

    if (validateFnfMods !== undefined)
      appSettings.validateFnfMods = validateFnfMods === true

    const showTerminalOutput =
      await storeService.getSetting('showTerminalOutput')

    await storeService.getSetting('showTerminalOutput')
    if (showTerminalOutput !== undefined)
      appSettings.showTerminalOutput = showTerminalOutput === true

    console.log('Loaded app settings:', appSettings)
  } catch (error) {
    console.error('Error loading app settings:', error)
  }
}

// Apply theme based on system or user preference
const applyTheme = async (themeValue: string | boolean) => {
  // Convert legacy boolean value to theme string
  let activeThemeValue: string
  if (typeof themeValue === 'boolean') {
    activeThemeValue = themeValue ? 'light' : 'dark'
  } else {
    activeThemeValue = themeValue
  }

  console.log('Applying theme:', activeThemeValue)

  try {
    // Apply theme using the theme service
    await themeService.applyTheme(activeThemeValue)

    // Apply accent color
    const storeService = StoreService.getInstance()
    const accentColor = await storeService.getSetting('accentColor')
    if (accentColor) {
      appSettings.accentColor = accentColor
      // Extract the color value if it's an object
      let colorValue = accentColor
      if (typeof accentColor === 'object' && accentColor.value) {
        colorValue = accentColor.value
      }
      // Apply the accent color to CSS custom properties
      document.documentElement.style.setProperty(
        '--q-primary',
        String(colorValue)
      )
      console.log(
        'Applied accent color during theme initialization:',
        colorValue
      )
    }

    // Check if we're running on Windows 11 for Mica effect
    const isWindows11 = await invoke<boolean>('is_windows_11')
    console.log('Is Windows 11:', isWindows11, 'Theme:', activeThemeValue)

    // Handle platform-specific styling
    if (!isWindows11) {
      // Apply solid theme styling for non-Windows 11 platforms
      handleNonWindows11Styling(activeThemeValue)
    } else {
      // Handle Windows 11 Mica effect
      await handleWindows11Styling(activeThemeValue)
    }
  } catch (error) {
    console.error('Failed to apply theme:', error)
    // Fallback to basic theme application
    fallbackThemeApplication(activeThemeValue)
  }
}

// Handle styling for non-Windows 11 platforms
const handleNonWindows11Styling = (themeValue: string) => {
  // Only apply solid-theme to light and dark themes
  if (themeValue === 'light' || themeValue === 'dark') {
    document.body.classList.add('solid-theme')
  } else {
    document.body.classList.remove('solid-theme')
  }

  // Remove transparent background styles
  document.documentElement.style.setProperty(
    '--transparent-bg-override',
    'none'
  )

  // Set background to solid color instead of transparent
  const bgColor = `var(--theme-bg)`

  document.body.style.removeProperty('background')
  document.body.style.backgroundColor = bgColor
  document
    .querySelector('.q-layout')
    ?.setAttribute('style', 'background: ' + bgColor + ' !important')
}

// Handle Windows 11 Mica effect styling
const handleWindows11Styling = async (themeValue: string) => {
  // Only light and dark themes should be transparent for Mica
  if (themeService.supportsWindowsMica(themeValue)) {
    document.body.classList.remove('solid-theme')
    document.documentElement.style.setProperty(
      '--transparent-bg-override',
      'transparent'
    )
    document.body.style.removeProperty('background')
    document.body.setAttribute('style', 'background: transparent !important')

    // Make sure q-layout is also transparent for Mica to work properly
    const qLayout = document.querySelector('.q-layout')
    if (qLayout) {
      qLayout.removeAttribute('style')
      qLayout.setAttribute('style', 'background: transparent !important')
    }

    // Apply Mica effect based on theme
    try {
      await invoke('change_mica_theme', {
        window: 'main',
        dark: themeValue !== 'light',
      })
      console.log(
        'Applied Mica theme effect:',
        themeValue !== 'light' ? 'dark' : 'light'
      )
    } catch (error) {
      console.error('Failed to apply Mica effect:', error)
    }
  } else {
    // For custom themes on Windows 11, use solid styling
    document.body.classList.remove('solid-theme')
    document.documentElement.style.setProperty(
      '--transparent-bg-override',
      'none'
    )

    // Set background to solid color based on the theme
    const bgColor = `var(--theme-bg)`

    document.body.style.removeProperty('background')
    document.body.style.backgroundColor = bgColor
    document
      .querySelector('.q-layout')
      ?.setAttribute('style', 'background: ' + bgColor + ' !important')

    // Remove Mica effect for non-standard themes
    try {
      await invoke('remove_mica_theme', {
        window: 'main',
      })
      console.log('Removed Mica effect for theme:', themeValue)
    } catch (error) {
      console.error('Failed to remove Mica effect:', error)
    }
  }
}

// Fallback theme application without theme service
const fallbackThemeApplication = (themeValue: string) => {
  console.warn('Using fallback theme application for:', themeValue)

  // Remove all theme classes
  document.body.classList.forEach(className => {
    if (className.endsWith('-theme')) {
      document.body.classList.remove(className)
    }
  })

  // Add the active theme class
  document.body.classList.add(`${themeValue}-theme`)
}

// Get the current system theme (light or dark)
const getSystemTheme = (): boolean => {
  return (
    window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: light)').matches
  )
}

// Handle system theme changes
const handleSystemThemeChange = async (event: MediaQueryListEvent) => {
  // Only react to system theme changes if that setting is enabled
  const useSystemTheme = await getUseSystemThemeSetting()
  if (useSystemTheme) {
    const isLightTheme = event.matches
    applyTheme(isLightTheme)
  }
}

// Get the current "use system theme" setting from the database
const getUseSystemThemeSetting = async (): Promise<boolean> => {
  try {
    const storeService = StoreService.getInstance()
    return await storeService.getSetting('useSystemTheme')
  } catch (error) {
    console.error('Error fetching useSystemTheme setting:', error)
    return true // Default to true on error
  }
}

// Get the manually set theme preference
const getThemePreference = async (): Promise<string> => {
  try {
    const storeService = StoreService.getInstance()

    // Get theme from the store
    const theme = await storeService.getSetting('theme')
    if (theme) {
      return theme
    }

    return 'dark' // Default to dark theme
  } catch (error) {
    console.error('Error fetching theme preference:', error)
    return 'dark' // Default to dark theme
  }
}

// Initialize the app
let cleanupEventListeners: (() => void) | undefined

onMounted(async () => {
  try {
    const currentWindow = getCurrentWindow()
    console.log('Current window:', currentWindow)
    currentWindow.show()
    // Update progress bar - Step 1: Initialize theme service
    initStatusText.value = t('app.notifications.initialization.theme_service')
    initProgress.value = 0.1

    // Initialize the theme service first
    await themeService.initialize() // Update progress bar - Step 2: Apply theme
    initStatusText.value = t('app.notifications.initialization.applying_theme')
    initProgress.value = 0.15

    // Apply initial theme based on system or user preference
    const useSystemThemeInitial = await getUseSystemThemeSetting()
    if (useSystemThemeInitial) {
      // If using system theme, apply light or dark based on system preference
      const isSystemLight = getSystemTheme()
      await applyTheme(isSystemLight ? 'light' : 'dark')
    } else {
      // If using custom theme, apply the saved theme directly
      const themeValue = await getThemePreference()
      await applyTheme(themeValue)
    }

    // Set up global function for handling URL clicks in v-html content
    ;(
      window as typeof window & { handleUrlClick: (url: string) => void }
    ).handleUrlClick = (url: string) => {
      openUrl(url)
    }

    // Update progress bar - Step 3: Initialize deep link handler
    initStatusText.value = t('app.notifications.initialization.deep_linking')
    initProgress.value = 0.2

    // Set up deep link handler
    onOpenUrl(async url => {
      console.log('Deep link received:', url[0])
      await getCurrentWindow().setFocus()

      // Process the deep link URL
      if (url[0].startsWith('flmod://')) {
        try {
          // Remove the protocol prefix
          const data = url[0].substring(8) // Remove "flmod://"

          // Split the URL by $ delimiter
          const parts = data.split('$')
          if (parts.length >= 4) {
            let downloadUrl = parts[0]
            const modType = parts[1]
            const modId = parseInt(parts[2])
            const archiveExt = parts[3]

            downloadUrl = downloadUrl.replace('https//', 'https://') // Fix URL format

            console.log('Parsed deep link:', {
              downloadUrl,
              modType,
              modId,
              archiveExt,
            })

            // Temp until I fix this
          } else {
            console.error('Invalid deep link format, missing required parts')
          }
        } catch (error) {
          console.error('Failed to process deep link:', error)
        }
      }
    })

    // Update progress bar - Step 4: Initialize database
    initStatusText.value = t('app.notifications.initialization.db_service')
    initProgress.value = 0.3

    // Initialize the database service
    const dbService = DatabaseService.getInstance()
    await dbService.initialize()

    // Make the database service available globally
    window.db = {
      // Direct access to the database service
      select: async (query: string, params?: any[]) => {
        const db = await dbService['db']
        return db.select(query, params)
      },
      execute: async (query: string, params?: any[]) => {
        const db = await dbService['db']
        return db.execute(query, params)
      },
      service: dbService,
    }

    // Update progress bar - Step 3: Load mods
    initStatusText.value = t('app.notifications.initialization.loading_mods')
    initProgress.value = 0.4 // Load mods from the database
    const mods = await dbService.getAllMods()
    console.log('Loaded mods from database:', mods)

    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`)
      // No need to sync with backend as the dbService handles this internally
    } else {
      console.log('No mods found in database')
    }

    // Update progress bar - Step 5: Load settings
    initStatusText.value = t(
      'app.notifications.initialization.loading_settings'
    )
    initProgress.value = 0.6

    // Load app settings
    await loadAppSettings()

    // Set up listener for missing mod paths
    await listen<string>('mod-path-missing', event => {
      console.log('Received mod-path-missing event:', event)
      missingModPath.value = event.payload
      showModPathMissingDialog.value = true
    })

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: light)')
    mediaQuery.addEventListener('change', handleSystemThemeChange)

    // Update progress bar - Step 6: Check for updates
    initStatusText.value = t(
      'app.notifications.initialization.checking_for_updates'
    )
    initProgress.value = 0.8

    // Check for updates
    try {
      const updateResult = await check()
      if (updateResult !== null) {
        // Update progress bar - Updates available
        initStatusText.value = t(
          'app.notifications.initialization.installing_update'
        )

        // Install the update if there is one
        let contentLength = 0
        await updateResult.downloadAndInstall(event => {
          switch (event.event) {
            case 'Started': {
              contentLength = event.data.contentLength ?? 0
              break
            }
            case 'Progress': {
              let downloaded = 0
              downloaded += event.data.chunkLength
              const percent = contentLength
                ? Math.round((downloaded / contentLength) * 100)
                : 0
              initProgress.value = 0.8 + percent / 1000
              break
            }
            case 'Finished':
              initProgress.value = 1.0
              break
          }
        })

        await relaunch()
      } else {
        console.log('No updates available')
      }
    } catch (error) {
      console.error('Failed to check for updates:', error)
    }

    // Update progress bar - Step 7: Complete
    initStatusText.value = t('app.notifications.initialization.ready')
    initProgress.value = 1.0

    // Hide the loading overlay after a small delay to ensure transitions are smooth
    setTimeout(() => {
      isInitializing.value = false
    }, 500)

    console.log('App initialized and database tables updated')
  } catch (error) {
    console.error('Failed to initialize database:', error)

    // Update progress bar with error state
    initStatusText.value = 'Error during initialization: ' + String(error)
    initProgress.value = 1.0

    // Hide the loading overlay after a delay
    setTimeout(() => {
      isInitializing.value = false
    }, 2000)
    onUnmounted(() => {
      // Clean up the event listener for system theme changes
      const mediaQuery = window.matchMedia('(prefers-color-scheme: light)')
      mediaQuery.removeEventListener('change', handleSystemThemeChange)

      // Clean up download event listeners
      if (typeof cleanupEventListeners === 'function') {
        cleanupEventListeners()
      }
    })
  }
})
</script>

<style>
:root {
  --red: #c03221;
  --orange: #fe621d;
  --yellow: #f7c548;
  --green: #35ce8d;
  --blue: #39dbce;
  --purple: #c490d1;
  --pink: #db2955;
}

html,
body {
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  color: var(--theme-text);
}
</style>

<style scoped>
.transparent-bg {
  background: transparent !important;
}

/* Loading overlay styles */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.loading-container {
  width: 400px;
  text-align: center;
  padding: 24px;
  border-radius: 12px;
  background-color: var(--theme-card);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  border: 2px solid var(--theme-border);
}

.loading-container h2 {
  margin-top: 0;
  margin-bottom: 24px;
}

.loading-container .text-caption {
  color: var(--theme-text-secondary, #aaaaaa);
  margin-top: 12px;
}

.cls-1 {
  fill: url(#linear-gradient-2);
}

.cls-2 {
  fill: url(#linear-gradient-10);
}

.cls-3 {
  fill: var(--q-primary);
}

.cls-4 {
  fill: var(--theme-border);
}

.cls-5 {
  fill: url(#linear-gradient-4);
}

.cls-6 {
  fill: url(#linear-gradient-3);
}

.cls-7 {
  fill: url(#linear-gradient-5);
}

.cls-8 {
  fill: url(#linear-gradient-8);
}

.cls-9 {
  fill: url(#linear-gradient-7);
}

.cls-10 {
  fill: url(#linear-gradient-9);
}

.cls-11 {
  fill: url(#linear-gradient-6);
}

.cls-12 {
  fill: url(#linear-gradient);
}
</style>
