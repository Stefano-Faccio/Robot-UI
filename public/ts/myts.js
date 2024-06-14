(function () {
    //The SVG icons
    var robot_svg = '<svg class="robot" style="width: 100%;height: 100%" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M320 0c17.7 0 32 14.3 32 32V96H472c39.8 0 72 32.2 72 72V440c0 39.8-32.2 72-72 72H168c-39.8 0-72-32.2-72-72V168c0-39.8 32.2-72 72-72H288V32c0-17.7 14.3-32 32-32zM208 384c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H208zm96 0c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H304zm96 0c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H400zM264 256a40 40 0 1 0 -80 0 40 40 0 1 0 80 0zm152 40a40 40 0 1 0 0-80 40 40 0 1 0 0 80zM48 224H64V416H48c-26.5 0-48-21.5-48-48V272c0-26.5 21.5-48 48-48zm544 0c26.5 0 48 21.5 48 48v96c0 26.5-21.5 48-48 48H576V224h16z"/></svg>';
    var market_svg = '<svg style="fill:#4870d8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M547.6 103.8L490.3 13.1C485.2 5 476.1 0 466.4 0H109.6C99.9 0 90.8 5 85.7 13.1L28.3 103.8c-29.6 46.8-3.4 111.9 51.9 119.4c4 .5 8.1 .8 12.1 .8c26.1 0 49.3-11.4 65.2-29c15.9 17.6 39.1 29 65.2 29c26.1 0 49.3-11.4 65.2-29c15.9 17.6 39.1 29 65.2 29c26.2 0 49.3-11.4 65.2-29c16 17.6 39.1 29 65.2 29c4.1 0 8.1-.3 12.1-.8c55.5-7.4 81.8-72.5 52.1-119.4zM499.7 254.9l-.1 0c-5.3 .7-10.7 1.1-16.2 1.1c-12.4 0-24.3-1.9-35.4-5.3V384H128V250.6c-11.2 3.5-23.2 5.4-35.6 5.4c-5.5 0-11-.4-16.3-1.1l-.1 0c-4.1-.6-8.1-1.3-12-2.3V384v64c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V384 252.6c-4 1-8 1.8-12.3 2.3z"/></svg>';
    var bank_svg = '<svg style="fill:#cfb78f" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M243.4 2.6l-224 96c-14 6-21.8 21-18.7 35.8S16.8 160 32 160v8c0 13.3 10.7 24 24 24H456c13.3 0 24-10.7 24-24v-8c15.2 0 28.3-10.7 31.3-25.6s-4.8-29.9-18.7-35.8l-224-96c-8-3.4-17.2-3.4-25.2 0zM128 224H64V420.3c-.6 .3-1.2 .7-1.8 1.1l-48 32c-11.7 7.8-17 22.4-12.9 35.9S17.9 512 32 512H480c14.1 0 26.5-9.2 30.6-22.7s-1.1-28.1-12.9-35.9l-48-32c-.6-.4-1.2-.7-1.8-1.1V224H384V416H344V224H280V416H232V224H168V416H128V224zM256 64a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"/></svg>';
    var bush_svg = '<svg style="fill:#00b300" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M272 96c-78.6 0-145.1 51.5-167.7 122.5c33.6-17 71.5-26.5 111.7-26.5h88c8.8 0 16 7.2 16 16s-7.2 16-16 16H288 216s0 0 0 0c-16.6 0-32.7 1.9-48.3 5.4c-25.9 5.9-49.9 16.4-71.4 30.7c0 0 0 0 0 0C38.3 298.8 0 364.9 0 440v16c0 13.3 10.7 24 24 24s24-10.7 24-24V440c0-48.7 20.7-92.5 53.8-123.2C121.6 392.3 190.3 448 272 448l1 0c132.1-.7 239-130.9 239-291.4c0-42.6-7.5-83.1-21.1-119.6c-2.6-6.9-12.7-6.6-16.2-.1C455.9 72.1 418.7 96 376 96L272 96z"/></svg>';
    var jollyBlock_svg = '<svg style="fill:#ff33cc" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M336.6 156.5c1.3 1.1 2.7 2.2 3.9 3.3c9.3 8.2 23 10.5 33.4 3.6l67.6-45.1c11.4-7.6 14.2-23.2 5.1-33.4C430 66.6 410.9 50.6 389.7 37.6c-11.9-7.3-26.9-1.4-32.1 11.6l-30.5 76.2c-4.5 11.1 .2 23.6 9.5 31.2zM328 36.8c5.1-12.8-1.6-27.4-15-30.5C294.7 2.2 275.6 0 256 0s-38.7 2.2-57 6.4C185.5 9.4 178.8 24 184 36.8l30.3 75.8c4.5 11.3 16.8 17.2 29 16c4.2-.4 8.4-.6 12.7-.6s8.6 .2 12.7 .6c12.1 1.2 24.4-4.7 29-16L328 36.8zM65.5 85c-9.1 10.2-6.3 25.8 5.1 33.4l67.6 45.1c10.3 6.9 24.1 4.6 33.4-3.6c1.3-1.1 2.6-2.3 4-3.3c9.3-7.5 13.9-20.1 9.5-31.2L154.4 49.2c-5.2-12.9-20.3-18.8-32.1-11.6C101.1 50.6 82 66.6 65.5 85zm314 137.1c.9 3.3 1.7 6.6 2.3 10c2.5 13 13 23.9 26.2 23.9h80c13.3 0 24.1-10.8 22.9-24c-2.5-27.2-9.3-53.2-19.7-77.3c-5.5-12.9-21.4-16.6-33.1-8.9l-68.6 45.7c-9.8 6.5-13.2 19.2-10 30.5zM53.9 145.8c-11.6-7.8-27.6-4-33.1 8.9C10.4 178.8 3.6 204.8 1.1 232c-1.2 13.2 9.6 24 22.9 24h80c13.3 0 23.8-10.8 26.2-23.9c.6-3.4 1.4-6.7 2.3-10c3.1-11.4-.2-24-10-30.5L53.9 145.8zM104 288H24c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24h80c13.3 0 24-10.7 24-24V312c0-13.3-10.7-24-24-24zm304 0c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24h80c13.3 0 24-10.7 24-24V312c0-13.3-10.7-24-24-24H408zM24 416c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24h80c13.3 0 24-10.7 24-24V440c0-13.3-10.7-24-24-24H24zm384 0c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24h80c13.3 0 24-10.7 24-24V440c0-13.3-10.7-24-24-24H408zM272 192c0-8.8-7.2-16-16-16s-16 7.2-16 16V464c0 8.8 7.2 16 16 16s16-7.2 16-16V192zm-64 32c0-8.8-7.2-16-16-16s-16 7.2-16 16V464c0 8.8 7.2 16 16 16s16-7.2 16-16V224zm128 0c0-8.8-7.2-16-16-16s-16 7.2-16 16V464c0 8.8 7.2 16 16 16s16-7.2 16-16V224z"/></svg>';
    var bin_svg = '<svg style="fill:#33552b" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"/></svg>';
    var garbage_svg = '<svg style="fill:#262626" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M192 96H320l47.4-71.1C374.5 14.2 366.9 0 354.1 0H157.9c-12.8 0-20.4 14.2-13.3 24.9L192 96zm128 32H192c-3.8 2.5-8.1 5.3-13 8.4l0 0 0 0C122.3 172.7 0 250.9 0 416c0 53 43 96 96 96H416c53 0 96-43 96-96c0-165.1-122.3-243.3-179-279.6c-4.8-3.1-9.2-5.9-13-8.4zM289.9 336l47 47c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0l-47-47-47 47c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l47-47-47-47c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l47 47 47-47c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-47 47z"/></svg>';
    var scarecrow_svg = '<svg style="fill:#ffcc66" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M456 0c-48.6 0-88 39.4-88 88v29.2L12.5 390.6c-14 10.8-16.6 30.9-5.9 44.9s30.9 16.6 44.9 5.9L126.1 384H259.2l46.6 113.1c5 12.3 19.1 18.1 31.3 13.1s18.1-19.1 13.1-31.3L311.1 384H352c1.1 0 2.1 0 3.2 0l46.6 113.2c5 12.3 19.1 18.1 31.3 13.1s18.1-19.1 13.1-31.3l-42-102C484.9 354.1 544 280 544 192V128v-8l80.5-20.1c8.6-2.1 13.8-10.8 11.6-19.4C629 52 603.4 32 574 32H523.9C507.7 12.5 483.3 0 456 0zm0 64a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"/></svg>';
    var coin_svg = '<svg style="fill:gold" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M504 256c0 137-111 248-248 248S8 393 8 256 119 8 256 8s248 111 248 248zm-141.7-35.3c4.9-33-20.2-50.7-54.6-62.6l11.1-44.7-27.2-6.8-10.9 43.5c-7.2-1.8-14.5-3.5-21.8-5.1l10.9-43.8-27.2-6.8-11.2 44.7c-5.9-1.3-11.7-2.7-17.4-4.1l0-.1-37.5-9.4-7.2 29.1s20.2 4.6 19.8 4.9c11 2.8 13 10 12.7 15.8l-12.7 50.9c.8 .2 1.7 .5 2.8 .9-.9-.2-1.9-.5-2.9-.7l-17.8 71.3c-1.3 3.3-4.8 8.4-12.5 6.5 .3 .4-19.8-4.9-19.8-4.9l-13.5 31.1 35.4 8.8c6.6 1.7 13 3.4 19.4 5l-11.3 45.2 27.2 6.8 11.2-44.7a1038.2 1038.2 0 0 0 21.7 5.6l-11.1 44.5 27.2 6.8 11.3-45.1c46.4 8.8 81.3 5.2 96-36.7 11.8-33.8-.6-53.3-25-66 17.8-4.1 31.2-15.8 34.7-39.9zm-62.2 87.2c-8.4 33.8-65.3 15.5-83.8 10.9l14.9-59.9c18.4 4.6 77.6 13.7 68.8 49zm8.4-87.7c-7.7 30.7-55 15.1-70.4 11.3l13.5-54.3c15.4 3.8 64.8 11 56.8 43z"/></svg>';
    var fire_svg = '<svg style="fill:#b30000" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-4.5-9-16-9.6-22.5-2l-25.2 29.3c-6.6 7.6-18.5 7.4-24.7-.5c-16.5-21-46-58.5-62.8-79.8c-6.3-8-18.3-8.1-24.7-.1c-33.8 42.5-50.8 69.3-50.8 99.4C112 375.4 162.6 416 225.7 416z"/></svg>';
    var building_svg = '<svg style="fill:#ea3e12" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M48 0C21.5 0 0 21.5 0 48V464c0 26.5 21.5 48 48 48h96V432c0-26.5 21.5-48 48-48s48 21.5 48 48v80h96c26.5 0 48-21.5 48-48V48c0-26.5-21.5-48-48-48H48zM64 240c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V240zm112-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V240c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272c-8.8 0-16-7.2-16-16V240zM80 96h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V112zM272 96h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16z"/></svg>';
    var tree_svg = '<svg style="fill:#2c5545" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M210.6 5.9L62 169.4c-3.9 4.2-6 9.8-6 15.5C56 197.7 66.3 208 79.1 208H104L30.6 281.4c-4.2 4.2-6.6 10-6.6 16C24 309.9 34.1 320 46.6 320H80L5.4 409.5C1.9 413.7 0 419 0 424.5c0 13 10.5 23.5 23.5 23.5H192v32c0 17.7 14.3 32 32 32s32-14.3 32-32V448H424.5c13 0 23.5-10.5 23.5-23.5c0-5.5-1.9-10.8-5.4-15L368 320h33.4c12.5 0 22.6-10.1 22.6-22.6c0-6-2.4-11.8-6.6-16L344 208h24.9c12.7 0 23.1-10.3 23.1-23.1c0-5.7-2.1-11.3-6-15.5L237.4 5.9C234 2.1 229.1 0 224 0s-10 2.1-13.4 5.9z"/></svg>';
    var crate_svg = '<svg style="fill:darkorange" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M176 56V96H336V56c0-4.4-3.6-8-8-8H184c-4.4 0-8 3.6-8 8zM128 96V56c0-30.9 25.1-56 56-56H328c30.9 0 56 25.1 56 56V96v32V480H128V128 96zM64 96H96V480H64c-35.3 0-64-28.7-64-64V160c0-35.3 28.7-64 64-64zM448 480H416V96h32c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64z"/></svg>';
    var fish_svg = '<svg style="fill:#fa8072" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M275.2 38.4c-10.6-8-25-8.5-36.3-1.5S222 57.3 224.6 70.3l9.7 48.6c-19.4 9-36.9 19.9-52.4 31.5c-15.3 11.5-29 23.9-40.7 36.3L48.1 132.4c-12.5-7.3-28.4-5.3-38.6 4.9S-3 163.3 4.2 175.9L50 256 4.2 336.1c-7.2 12.6-5 28.4 5.3 38.6s26.1 12.2 38.6 4.9l93.1-54.3c11.8 12.3 25.4 24.8 40.7 36.3c15.5 11.6 33 22.5 52.4 31.5l-9.7 48.6c-2.6 13 3.1 26.3 14.3 33.3s25.6 6.5 36.3-1.5l77.6-58.2c54.9-4 101.5-27 137.2-53.8c39.2-29.4 67.2-64.7 81.6-89.5c5.8-9.9 5.8-22.2 0-32.1c-14.4-24.8-42.5-60.1-81.6-89.5c-35.8-26.8-82.3-49.8-137.2-53.8L275.2 38.4zM384 256a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z"/></svg>';
    var water_svg = '<svg style="fill:#0099cc" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M192 512C86 512 0 426 0 320C0 228.8 130.2 57.7 166.6 11.7C172.6 4.2 181.5 0 191.1 0h1.8c9.6 0 18.5 4.2 24.5 11.7C253.8 57.7 384 228.8 384 320c0 106-86 192-192 192zM96 336c0-8.8-7.2-16-16-16s-16 7.2-16 16c0 61.9 50.1 112 112 112c8.8 0 16-7.2 16-16s-7.2-16-16-16c-44.2 0-80-35.8-80-80z"/></svg>';
    var rock_svg = '<svg style="fill:darkgray" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M252.4 103.8l27 48c2.8 5 8.2 8.2 13.9 8.2l53.3 0c5.8 0 11.1-3.1 13.9-8.2l27-48c2.7-4.9 2.7-10.8 0-15.7l-27-48c-2.8-5-8.2-8.2-13.9-8.2H293.4c-5.8 0-11.1 3.1-13.9 8.2l-27 48c-2.7 4.9-2.7 10.8 0 15.7zM68.3 87C43.1 61.8 0 79.7 0 115.3V432c0 44.2 35.8 80 80 80H396.7c35.6 0 53.5-43.1 28.3-68.3L68.3 87zM504.2 403.6c4.9 2.7 10.8 2.7 15.7 0l48-27c5-2.8 8.2-8.2 8.2-13.9V309.4c0-5.8-3.1-11.1-8.2-13.9l-48-27c-4.9-2.7-10.8-2.7-15.7 0l-48 27c-5 2.8-8.2 8.2-8.2 13.9v53.3c0 5.8 3.1 11.1 8.2 13.9l48 27zM192 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM384 288a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"/></svg>';
    //-------------------------------------------------------
    //Energy bar
    var bar = new ldBar(".ldBar", {}); //Energy bar
    //To change dynamically the size of the grid and the backpack
    var updateDims = function () {
        var grid_container = document.getElementById('grid_container');
        var backpack_container = document.getElementById('backpack_container');
        var grid_style = window.getComputedStyle(grid_container);
        var backpack_style = window.getComputedStyle(backpack_container);
        var grid_width = window.innerHeight - parseInt(grid_style.marginTop, 10) - parseInt(grid_style.marginBottom, 10);
        var backpack_width = ((window.innerWidth - grid_width) / 2) - parseInt(backpack_style.marginLeft, 10) - parseInt(backpack_style.marginRight, 10);
        var max_backpack_width = ((grid_width * 1.95) / BACKPACK_SIZE);
        grid_container.style.width = "" + grid_width + "px";
        backpack_container.style.width = "" + backpack_width + "px";
        backpack_container.style.maxWidth = "" + max_backpack_width + "px";
        backpack_container.style.minWidth = '50px';
    };
    //The websocket functions to open and close the connection
    var ws = null;
    var ws_open = function () {
        //Open a websocket connection
        ws = new WebSocket('ws://127.0.0.1:8000//ws');
        ws.onopen = function () {
            console.log('WebSocket Client Connected');
            //Cambio il testo del bottone
            var my_button = document.getElementById('my_button');
            my_button.innerText = 'Stop Game';
            //Cambio la classe del bottone aggiugendo bnt-danger e rimuovendo btn-primary
            my_button.classList.remove('btn-primary');
            my_button.classList.add('btn-danger');
            //Cambio la funzione di click del bottone
            my_button.removeEventListener('click', ws_open);
            my_button.addEventListener('click', ws_close);
            //Start a game
            game_functions();
        };
        ws.onmessage = function (e) {
            var _a, _b, _c, _d, _e, _f;
            //Convert the string to JSON
            var jsonResponse = JSON.parse(e.data);
            var energy = (_a = jsonResponse.energy) !== null && _a !== void 0 ? _a : 0;
            var message = (_b = jsonResponse.test_string) !== null && _b !== void 0 ? _b : '';
            var world = (_c = jsonResponse.world) !== null && _c !== void 0 ? _c : [];
            var backpack = (_d = jsonResponse.backpack) !== null && _d !== void 0 ? _d : {};
            var ui_coordinate = (_e = jsonResponse.ui_coordinate) !== null && _e !== void 0 ? _e : [0, 0];
            var grid_coordinate = (_f = jsonResponse.grid_coordinate) !== null && _f !== void 0 ? _f : [0, 0];
            //Aggiorno l'enegia
            bar.set(energy);
            //Aggiorno le coordinate
            var p_coordinate = document.getElementById('p-coordinate');
            p_coordinate.innerText = "[".concat(grid_coordinate[0], ", ").concat(grid_coordinate[1], "] -> [").concat(ui_coordinate[0], ", ").concat(ui_coordinate[1], "] ").concat(message);
            //Aggioro lo zaino
            for (var item in backpack) {
                if (item.toLowerCase() !== 'none') {
                    var item_count_div = document.getElementById("item-count-".concat(item));
                    item_count_div.innerText = backpack[item];
                }
            }
            //Aggiorno la griglia
            // Creo un set con tutti gli id delle celle
            var new_cells = new Set(CELLS);
            //Per ogni cella occupata nel mondo
            for (var i = 0; i < world.length; i++) {
                var id = world[i].id;
                var tile_type = world[i].tile_type;
                var elevation = world[i].elevation;
                var content = world[i].content;
                var cell = document.getElementById("cell-".concat(id));
                //Rimuovo l'id dalla lista delle celle
                new_cells.delete(id);
                //Pulisco il contenuto
                cell.innerHTML = '';
                cell.style.background = '';
                if (typeof tile_type !== 'object') {
                    cell.style.backgroundColor = selectBackground(tile_type);
                } //Case teleport
                else if ("Teleport" in tile_type) {
                    if (tile_type["Teleport"])
                        cell.style.background = 'radial-gradient(#a542f1, #6a37c4, #020107)';
                    else
                        cell.style.background = 'radial-gradient(#6a37c4, #231937, #020107)';
                }
                else
                    console.log('Tile type not found');
                //Se la cella ha un contenuto mostro una svg con il contenuto
                if (content != null && content !== 'None') {
                    //Se è un oggetto
                    if (typeof content === 'object') {
                        var content_type = Object.keys(content)[0];
                        var content_numer = content[content_type];
                        //Modifico il contenuto
                        cell.innerHTML = selectContent(content_type + "");
                        //Ridimensiono il contenuto in base al numero di oggetti
                        //edit: non mi piace - lascio la dimensione standard
                        /*
                        let svg = cell.querySelector('svg');
                        let dim = Math.min((50 + (5 * content_numer)), 95);
                        svg.style.width = dim+'%';
                        svg.style.height = dim+'%';
                         */
                    }
                    else //Se è una stringa
                     {
                        cell.innerHTML = selectContent(content + "");
                    }
                }
            }
            //Pulisco le celle rimanenti
            new_cells.forEach(function (id) {
                var cell = document.getElementById("cell-".concat(id));
                //cell.style.backgroundColor = '#262626';
                cell.style.background = 'repeating-conic-gradient(#333333 10%, black 20%)';
                cell.innerHTML = '';
            });
            //Mostro il robot
            var robot_cell = document.getElementById("cell-".concat(ui_coordinate[0] * GRID_SIZE + ui_coordinate[1]));
            robot_cell.innerHTML = robot_svg;
        };
        ws.onclose = function () {
            console.log('WebSocket Client Closed');
            //Resetto il bottone
            var my_button = document.getElementById('my_button');
            my_button.innerText = 'Start Game';
            //Cambio la classe del bottone aggiugendo bnt-primary e rimuovendo btn-danger
            my_button.classList.remove('btn-danger');
            my_button.classList.add('btn-primary');
            //Cambio la funzione di click del bottone
            my_button.removeEventListener('click', ws_close);
            my_button.addEventListener('click', ws_open);
        };
        ws.onerror = function (e) {
            console.log('WebSocket Client Error');
        };
    };
    var ws_close = function () {
        if (ws) {
            ws.close();
        }
        else {
            console.log('WebSocket Client already closed');
        }
    };
    var game_functions = function () {
        fetch('/start', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({})
        }).then(function (response) {
            if (response.ok) {
                return response.json();
            }
            else
                throw new Error('Request failed!');
        }, function (networkError) { return console.log(networkError.message); })
            .then(function (jsonResponse) {
            console.log("Game started");
        });
    };
    //To fill the backpack with the items
    var fillBackpack = function () {
        //Riempio il backpack
        BACKPACK.forEach(function (item) {
            var item_div = document.getElementById("item-" + item + "");
            var item_count_div = document.getElementById("item-count-" + item + "");
            item_count_div.innerText = '0';
            //Per ogni item diverso metto un'icona diversa
            switch (item) {
                case 'Market':
                    item_div.innerHTML += market_svg;
                    break;
                case 'Bank':
                    item_div.innerHTML += bank_svg;
                    break;
                case 'Bush':
                    item_div.innerHTML += bush_svg;
                    break;
                case 'JollyBlock':
                    item_div.innerHTML += jollyBlock_svg;
                    break;
                case 'Bin':
                    item_div.innerHTML += bin_svg;
                    break;
                case 'Garbage':
                    item_div.innerHTML += garbage_svg;
                    break;
                case 'Scarecrow':
                    item_div.innerHTML += scarecrow_svg;
                    break;
                case 'Coin':
                    item_div.innerHTML += coin_svg;
                    break;
                case 'Fire':
                    item_div.innerHTML += fire_svg;
                    break;
                case 'Building':
                    item_div.innerHTML += building_svg;
                    break;
                case 'Tree':
                    item_div.innerHTML += tree_svg;
                    break;
                case 'Crate':
                    item_div.innerHTML += crate_svg;
                    break;
                case 'Fish':
                    item_div.innerHTML += fish_svg;
                    break;
                case 'Water':
                    item_div.innerHTML += water_svg;
                    break;
                case 'Rock':
                    item_div.innerHTML += rock_svg;
                    break;
                default:
                    console.log('Item not found');
                    break;
            }
        });
    };
    var showActiveTheme = function (theme, focus) {
        if (focus === void 0) { focus = false; }
        var themeSwitcher = document.querySelector('#bd-theme');
        if (!themeSwitcher) {
            return;
        }
        var themeSwitcherText = document.querySelector('#bd-theme-text');
        var activeThemeIcon = document.querySelector('.theme-icon-active use');
        var btnToActive = document.querySelector("[data-bs-theme-value=\"".concat(theme, "\"]"));
        var svgOfActiveBtn = btnToActive.querySelector('svg use').getAttribute('href');
        document.querySelectorAll('[data-bs-theme-value]').forEach(function (element) {
            element.classList.remove('active');
            element.setAttribute('aria-pressed', 'false');
        });
        btnToActive.classList.add('active');
        btnToActive.setAttribute('aria-pressed', 'true');
        activeThemeIcon.setAttribute('href', svgOfActiveBtn);
        var themeSwitcherLabel = "".concat(themeSwitcherText.textContent, " (").concat(btnToActive.dataset.bsThemeValue, ")");
        themeSwitcher.setAttribute('aria-label', themeSwitcherLabel);
        if (focus) {
            themeSwitcher.focus();
        }
    };
    //Function to select the background of the cell
    var selectBackground = function (tile_type) {
        var background = '';
        switch (tile_type) {
            case 'DeepWater':
                background = 'DarkBlue';
                break;
            case 'ShallowWater':
                background = 'Aqua';
                break;
            case 'Sand':
                background = '#e6e600';
                break;
            case 'Grass':
                background = '#9ACD32';
                break;
            case 'Street':
                background = 'SlateGrey';
                break;
            case 'Hill':
                background = 'Olive';
                break;
            case 'Mountain':
                background = 'Sienna';
                break;
            case 'Snow':
                background = 'Snow';
                break;
            case 'Lava':
                background = 'Red';
                break;
            case 'Wall':
                background = 'DarkSlateGrey';
                break;
            default:
                console.log('Tile type not found: ' + tile_type);
                break;
        }
        return background;
    };
    //Function to select the content of the cell
    var selectContent = function (content) {
        var newHTML = '';
        switch (content) {
            case 'Rock':
                newHTML = rock_svg;
                break;
            case 'Tree':
                newHTML = tree_svg;
                break;
            case 'Garbage':
                newHTML = garbage_svg;
                break;
            case 'Fire':
                newHTML = fire_svg;
                break;
            case 'Coin':
                newHTML = coin_svg;
                break;
            case 'Bin':
                newHTML = bin_svg;
                break;
            case 'Crate':
                newHTML = crate_svg;
                break;
            case 'Bank':
                newHTML = bank_svg;
                break;
            case 'Water':
                newHTML = water_svg;
                break;
            case 'Market':
                newHTML = market_svg;
                break;
            case 'Fish':
                newHTML = fish_svg;
                break;
            case 'Building':
                newHTML = building_svg;
                break;
            case 'Bush':
                newHTML = bush_svg;
                break;
            case 'JollyBlock':
                newHTML = jollyBlock_svg;
                break;
            case 'Scarecrow':
                newHTML = scarecrow_svg;
                break;
            case 'None':
                console.log('No content');
                break;
            default:
                console.log('Content not found: ' + content);
                break;
        }
        return newHTML;
    };
    // Functions for the theme switcher
    var getStoredTheme = function () { return localStorage.getItem('theme'); };
    var setStoredTheme = function (theme) { return localStorage.setItem('theme', theme); };
    var getPreferredTheme = function () {
        var storedTheme = getStoredTheme();
        if (storedTheme) {
            return storedTheme;
        }
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    };
    var setTheme = function (theme) {
        if (theme === 'auto') {
            document.documentElement.setAttribute('data-bs-theme', (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'));
        }
        else {
            document.documentElement.setAttribute('data-bs-theme', theme);
        }
    };
    //-------------------------------------------------------
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function () {
        var storedTheme = getStoredTheme();
        if (storedTheme !== 'light' && storedTheme !== 'dark') {
            setTheme(getPreferredTheme());
        }
    });
    window.addEventListener('DOMContentLoaded', function () {
        showActiveTheme(getPreferredTheme());
        document.querySelectorAll('[data-bs-theme-value]')
            .forEach(function (toggle) {
            toggle.addEventListener('click', function () {
                var theme = toggle.getAttribute('data-bs-theme-value');
                setStoredTheme(theme);
                setTheme(theme);
                showActiveTheme(theme, true);
            });
        });
    });
    window.addEventListener('resize', function () {
        updateDims();
    });
    document.getElementById('my_button').addEventListener('click', ws_open);
    setTheme(getPreferredTheme());
    updateDims();
    fillBackpack();
})();
