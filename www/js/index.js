window.addEventListener("beforeunload", async (event) => {
  console.log("before unload called");

  // even with preventDefault, a GET fetch doesn't seem to go out on chromium
  await fetch("localhost:8000", {
    method: 'POST',
    keepalive: true,
  });

  console.log("about to prevent default");
  event.preventDefault();  // Necessary for some browsers
});

// addEventListener('beforeunload', () => {
//   let formData = new FormData();
//   formData.append('id', 0);
//   formData.append('aksi', 'CANCEL');
//   formData.append('table', 'offday');

//   fetch(`http://localhost:8000/`, {
//       method: 'POST',
//       headers: {
//         'Content-Type': 'application/json',
//       //     //'Authorization': `Bearer ${accessToken}`
//       },
//       body: JSON.stringify(formData),
//       keepalive:true,
//   })
//   .then(response => {
//       console.log("got response from fetch");
//       if (!response.ok) {
//           throw new Error('Network response was not ok');
//       }

//       console.log("response is OK");
//       return response.json();
//   })
//   .then(data => {
//       console.log("about to check for crudModal");
//       $("#crudModal").modal("hide");
//   })
//   .catch(error => {
//       console.log('some kind of error', error);
//      
//       // Handle error
//       // if (error.status === 422) {
//       //   console.log("it's a 422");
//       //     $('.is-invalid').removeClass('is-invalid');
//       //     $('.invalid-feedback').remove();
//       //     setErrorMessages(form, error.responseJSON.errors);
//       // } else {
//       //     //showDialog(error.responseJSON);
//       // }
//   });
// });
