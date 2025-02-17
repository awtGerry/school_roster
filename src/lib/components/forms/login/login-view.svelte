<script lang="ts">
  let username = "";
  let password = "";
  let error = "";

  async function handleSubmit(event: Event) {
    event.preventDefault();

    const res = await fetch("/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });

    const data = await res.json();
    if (!res.ok) {
      error = data.error;
    } else {
      window.location.href = "/dashboard"; // Redirigir tras el login
    }
  }
</script>

<div class="container">
  <h2>Iniciar Sesion</h2>
  <form on:submit={handleSubmit}>
    <label for="username">Usuario</label>
    <input id="username" type="text" bind:value={username} required />

    <label for="password">Contraseña</label>
    <input id="password" type="password" bind:value={password} required />

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <button type="submit" class="login-btn">Ingresar</button>

    <button type="button" class="login-with-google-btn">
      Continuar con Google      
    </button>
  </form>
</div>

<style lang="scss">
  @use "../../../../styles/variables";

  .container {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 500px;
    padding: 40px;
    background: variables.$white-background;
    border-radius: 10px;
    box-shadow: 0px 6px 15px rgba(0, 0, 0, 0.2); // Sombra más pronunciada

    h2 {
      text-align: center;
    }

    form {
      display: flex;
      flex-direction: column;

      label {
        margin: 10px 0 5px;
      }

      input {
        padding: 10px;
        width: 478px;
        border: 1px solid #ccc;
        border-radius: 5px;
      }

      .error {
        color: red;
        text-align: center;
        margin-top: 10px;
      }

      .login-btn {
        margin-top: 15px;
        padding: 12px 16px 12px 42px;
        background: variables.$black;
        color: variables.$white-hard;
        border: none;
        border-radius: 3px;
        cursor: pointer;
        font-size: 16px;

        transition:
          background-color 0.3s,
          box-shadow 0.3s;

        &:hover {
          box-shadow:
            0 -1px 0 rgba(0, 0, 0, 0.04),
            0 4px 4px rgba(0, 0, 0, 0.25);
        }

        &:active {
          background-color: #eeeeee;
        }

        &:focus {
          outline: none;
          box-shadow:
            0 -1px 0 rgba(0, 0, 0, 0.04),
            0 2px 4px rgba(0, 0, 0, 0.25),
            0 0 0 3px variables.$white-overlay;
        }
      }

      .login-with-google-btn {
        margin-top: 12px;
        cursor: pointer;

        padding: 12px 16px 12px 42px;
        border: none;
        border-radius: 3px;
        box-shadow:
          0 -1px 0 rgba(0, 0, 0, 0.04),
          0 1px 1px rgba(0, 0, 0, 0.25);

        color: #757575;
        font-size: 14px;
        font-weight: 500;

        background-image: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTgiIGhlaWdodD0iMTgiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGcgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj48cGF0aCBkPSJNMTcuNiA5LjJsLS4xLTEuOEg5djMuNGg0LjhDMTMuNiAxMiAxMyAxMyAxMiAxMy42djIuMmgzYTguOCA4LjggMCAwIDAgMi42LTYuNnoiIGZpbGw9IiM0Mjg1RjQiIGZpbGwtcnVsZT0ibm9uemVybyIvPjxwYXRoIGQ9Ik05IDE4YzIuNCAwIDQuNS0uOCA2LTIuMmwtMy0yLjJhNS40IDUuNCAwIDAgMS04LTIuOUgxVjEzYTkgOSAwIDAgMCA4IDV6IiBmaWxsPSIjMzRBODUzIiBmaWxsLXJ1bGU9Im5vbnplcm8iLz48cGF0aCBkPSJNNCAxMC43YTUuNCA1LjQgMCAwIDEgMC0zLjRWNUgxYTkgOSAwIDAgMCAwIDhsMy0yLjN6IiBmaWxsPSIjRkJCQzA1IiBmaWxsLXJ1bGU9Im5vbnplcm8iLz48cGF0aCBkPSJNOSAzLjZjMS4zIDAgMi41LjQgMy40IDEuM0wxNSAyLjNBOSA5IDAgMCAwIDEgNWwzIDIuNGE1LjQgNS40IDAgMCAxIDUtMy43eiIgZmlsbD0iI0VBNDMzNSIgZmlsbC1ydWxlPSJub256ZXJvIi8+PHBhdGggZD0iTTAgMGgxOHYxOEgweiIvPjwvZz48L3N2Zz4=);
        background-color: white;
        background-repeat: no-repeat;
        background-position: 12px 11px;

        transition:
          background-color 0.3s,
          box-shadow 0.3s;

        &:hover {
          box-shadow:
            0 -1px 0 rgba(0, 0, 0, 0.04),
            0 2px 4px rgba(0, 0, 0, 0.25);
        }

        &:active {
          background-color: #eeeeee;
        }

        &:focus {
          outline: none;
          box-shadow:
            0 -1px 0 rgba(0, 0, 0, 0.04),
            0 2px 4px rgba(0, 0, 0, 0.25),
            0 0 0 3px variables.$white-overlay;
        }

        &:disabled {
          filter: grayscale(100%);
          background-color: variables.$gray;
          box-shadow:
            0 -1px 0 rgba(0, 0, 0, 0.04),
            0 1px 1px rgba(0, 0, 0, 0.25);
          cursor: not-allowed;
        }
      }
    }

    .message {
      text-align: center;
      margin-top: 15px;
      font-size: 16px;
      color: #333;

      a {
        color: #ff1e00;
        text-decoration: none;
        font-weight: bold;

        &:hover {
          color: #0056b3;
          text-decoration: underline;
        }
      }
    }
  }
</style>
