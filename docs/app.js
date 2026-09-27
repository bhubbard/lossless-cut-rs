document.addEventListener('DOMContentLoaded', () => {
  const opType = document.getElementById('operation-type');
  const inputFile = document.getElementById('input-file');
  const startTime = document.getElementById('start-time');
  const endTime = document.getElementById('end-time');
  const startGroup = document.getElementById('start-group');
  const endGroup = document.getElementById('end-group');
  const commandEl = document.getElementById('generated-command');
  const copyBtn = document.getElementById('copy-btn');

  function updateCommand() {
    const mode = opType.value;
    const file = (inputFile.value || 'video.mp4').trim();
    const start = (startTime.value || '00:00:00').trim();
    const end = (endTime.value || '00:01:00').trim();

    if (mode === 'trim') {
      startGroup.style.display = 'block';
      endGroup.style.display = 'block';
      commandEl.textContent = `lossless-cut trim -i ${file} -s ${start} -e ${end} -o trimmed_${file}`;
    } else if (mode === 'batch') {
      startGroup.style.display = 'none';
      endGroup.style.display = 'none';
      commandEl.textContent = `lossless-cut batch -i ${file} -p project.llc -o ./cut_output --merge`;
    } else if (mode === 'merge') {
      startGroup.style.display = 'none';
      endGroup.style.display = 'none';
      commandEl.textContent = `lossless-cut merge -i clip1.mp4 clip2.mp4 clip3.mp4 -o merged_output.mp4`;
    } else if (mode === 'extract-audio') {
      startGroup.style.display = 'none';
      endGroup.style.display = 'none';
      commandEl.textContent = `lossless-cut extract-audio -i ${file} -o audio.m4a -t 0`;
    }
  }

  [opType, inputFile, startTime, endTime].forEach(el => {
    el.addEventListener('input', updateCommand);
    el.addEventListener('change', updateCommand);
  });

  copyBtn.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(commandEl.textContent);
      copyBtn.textContent = 'Copied!';
      setTimeout(() => {
        copyBtn.textContent = 'Copy';
      }, 2000);
    } catch (e) {
      console.error(e);
    }
  });

  updateCommand();
});
