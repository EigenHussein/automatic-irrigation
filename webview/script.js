// "use strict";
// exports.__esModule = true;
// var Chart_js_1 = require("./Chart.js");
// var xValues = [100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
// new Chart_js_1["default"]("myChart", {
//     type: "line",
//     data: {
//         labels: xValues,
//         datasets: [{
//                 data: [860, 1140, 1060, 1060, 1070, 1110, 1330, 2210, 7830, 2478],
//                 borderColor: "red",
//                 fill: false
//             }, {
//                 data: [1600, 1700, 1700, 1900, 2000, 2700, 4000, 5000, 6000, 7000],
//                 borderColor: "green",
//                 fill: false
//             }, {
//                 data: [300, 700, 2000, 5000, 6000, 4000, 2000, 1000, 200, 100],
//                 borderColor: "blue",
//                 fill: false
//             }]
//     },
//     options: {
//         legend: { display: false }
//     }
// });


const ctx = document.getElementById('myChart').getContext('2d');

// Generate data points based on your chosen curve (adjust x values for desired range)
const data = {
  labels: ['0', '1', '2', '3', '4', '5'],
  datasets: [{
    label: 'My Curve',
    data: [0, Math.sin(1), Math.sin(2), Math.sin(3), Math.sin(4), Math.sin(5)],
    borderColor: 'rgb(255, 99, 132)',
    fill: false,
  }]
};

const chart = new Chart(ctx, {
  type: 'line',
  data: data,
  options: {
    // Add any desired customization options for the chart
  }
});