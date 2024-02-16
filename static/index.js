const propState = {
  x: 0.0,
  y: 0.0
};

const dPadState = {
  up: false,
  down: false,
  left: false,
  right: false,
  center: false,
};

document.addEventListener('DOMContentLoaded', () => {
  const buttons = document.querySelectorAll('.d-pad-button');

  for (button of buttons) {
    button.addEventListener('touchstart', (e) => {
      e.preventDefault();
      const direction = e.srcElement.getAttribute('data-direction');
      dPadState[direction] = true;
      handlePropEvent();
    });

    //button.addEventListener('mousedown', (e) => {
    //  e.preventDefault();
    //  const direction = e.srcElement.getAttribute('data-direction');
    //  dPadState[direction] = true;
    //  handlePropEvent();
    //});

    button.addEventListener('touchend', (e) => {
      e.preventDefault();
      const direction = e.srcElement.getAttribute('data-direction');
      dPadState[direction] = false;
      handlePropEvent();
    });

    //button.addEventListener('mouseup', (e) => {
    //  e.preventDefault();
    //  const direction = e.srcElement.getAttribute('data-direction');
    //  dPadState[direction] = false;
    //  handlePropEvent();
    //});
  }
});

const handlePropEvent = (eventDirection) => {
  updatePropState();
  requestPropState();
}

const setDPadButtonEnable = (direction) => {
  dPadState[direction] = true;
  console.log(dPadState);
}

const setDPadButtonDisable = (direction) => {
  dPadState[direction] = false;
  console.log(dPadState);
}

const updatePropState = () => {
  if (dPadState.left) {
    propState.x = -1.0;
  } else if (dPadState.right) {
    propState.x = 1.0;
  } else {
    propState.x = 0.0;
  }

  if (dPadState.up) {
    propState.y = 1.0;
  } else {
    propState.y = 0.0;
  }

  if (dPadState.center) {
    propState.x = 0.0;
    propState.y = 0.0;
  }
}

const requestPropState = () => {
  const options = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(propState)
  };

  fetch('/api/propulsion', options)
    .then(res => {
      if (!res.ok) {
        throw new Error('Bad response');
      }
    })
    .catch(error => {
      console.error(`Bad response on prop request: ${error}`);
    });
}
