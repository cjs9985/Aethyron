
import { useEffect, useRef, useState } from "react";
import * as THREE from "three";

type NodeDefinition = {
  name: string;
  position: THREE.Vector3;
  color: number;
};
type Agent = {
  name: string;
  role: string;
};

const nodes: NodeDefinition[] = [
  {
    name: "PLANNER",
    position: new THREE.Vector3(-3.2, 1.4, 0),
    color: 0x4f8cff,
  },
  {
    name: "TOOLS",
    position: new THREE.Vector3(3.2, 1.4, 0),
    color: 0x8b6cff,
  },
  {
    name: "MEMORY",
    position: new THREE.Vector3(0, -2.2, 0),
    color: 0x35c9a5,
  },
];

function createNode(
  scene: THREE.Scene,
  definition: NodeDefinition
): THREE.Group {
  const group = new THREE.Group();
  group.position.copy(definition.position);

  const geometry = new THREE.IcosahedronGeometry(0.55, 1);

  const material = new THREE.MeshStandardMaterial({
    color: definition.color,
    emissive: definition.color,
    emissiveIntensity: 1.2,
    wireframe: true,
  });

  const mesh = new THREE.Mesh(geometry, material);
  group.add(mesh);

  const glowGeometry = new THREE.SphereGeometry(0.18, 16, 16);

  const glowMaterial = new THREE.MeshBasicMaterial({
    color: definition.color,
  });

  const glow = new THREE.Mesh(glowGeometry, glowMaterial);
  group.add(glow);

  scene.add(group);

  return group;
}

function createConnection(
  scene: THREE.Scene,
  start: THREE.Vector3,
  end: THREE.Vector3
): THREE.Line {
  const geometry = new THREE.BufferGeometry().setFromPoints([
    start,
    end,
  ]);

  const material = new THREE.LineBasicMaterial({
    color: 0x29456f,
    transparent: true,
    opacity: 0.65,
  });

  const line = new THREE.Line(geometry, material);
  scene.add(line);

  return line;
}

function App() {
  const mountRef = useRef<HTMLDivElement>(null);

  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [agents, setAgents] = useState<Agent[]>([]);

  useEffect(() => {
    const mount = mountRef.current;

    if (!mount) return;
    fetch("http://127.0.0.1:3000/agents")
  .then((response) => {
    if (!response.ok) {
      throw new Error("Failed to fetch agents");
    }

    return response.json();
  })
  .then((data: Agent[]) => {
    setAgents(data);
  })
  .catch((error) => {
    console.error("Aethyron API connection failed:", error);
  });

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x03050a);

    const camera = new THREE.PerspectiveCamera(
      55,
      window.innerWidth / window.innerHeight,
      0.1,
      100
    );

    camera.position.set(0, 1.5, 10);
    camera.lookAt(0, 0, 0);

    const renderer = new THREE.WebGLRenderer({
      antialias: true,
    });

    renderer.setPixelRatio(
      Math.min(window.devicePixelRatio, 2)
    );

    renderer.setSize(
      window.innerWidth,
      window.innerHeight
    );

    mount.appendChild(renderer.domElement);

    // Lighting
    const ambientLight = new THREE.AmbientLight(
      0xffffff,
      0.7
    );

    scene.add(ambientLight);

    const coreLight = new THREE.PointLight(
      0x397dff,
      30,
      15
    );

    coreLight.position.set(0, 0, 2);
    scene.add(coreLight);

    // Core
    const coreGroup = new THREE.Group();
    scene.add(coreGroup);

    const coreGeometry =
      new THREE.IcosahedronGeometry(1.45, 2);

    const coreMaterial =
      new THREE.MeshStandardMaterial({
        color: 0x357cff,
        emissive: 0x0b2d66,
        emissiveIntensity: 1.8,
        wireframe: true,
      });

    const core = new THREE.Mesh(
      coreGeometry,
      coreMaterial
    );

    coreGroup.add(core);

    const innerGeometry =
      new THREE.IcosahedronGeometry(0.65, 2);

    const innerMaterial =
      new THREE.MeshBasicMaterial({
        color: 0x7db2ff,
        wireframe: true,
        transparent: true,
        opacity: 0.8,
      });

    const innerCore = new THREE.Mesh(
      innerGeometry,
      innerMaterial
    );

    coreGroup.add(innerCore);

    // Core orbital rings
    const ringGeometry =
      new THREE.TorusGeometry(2.05, 0.012, 8, 128);

    const ringMaterial =
      new THREE.MeshBasicMaterial({
        color: 0x397dff,
        transparent: true,
        opacity: 0.45,
      });

    const ringA = new THREE.Mesh(
      ringGeometry,
      ringMaterial
    );

    ringA.rotation.x = Math.PI / 2.5;
    coreGroup.add(ringA);

    const ringB = new THREE.Mesh(
      ringGeometry,
      ringMaterial.clone()
    );

    ringB.rotation.y = Math.PI / 2.5;
    coreGroup.add(ringB);

    // System nodes
    const nodeGroups = nodes.map((node) =>
      createNode(scene, node)
    );

    // Connections
    const corePosition = new THREE.Vector3(0, 0, 0);

    nodes.forEach((node) => {
  createConnection(scene, corePosition, node.position);
});

    // Mission ring
    const missionRingGeometry =
      new THREE.TorusGeometry(
        4.5,
        0.018,
        8,
        160
      );

    const missionRingMaterial =
      new THREE.MeshBasicMaterial({
        color: 0x203b64,
        transparent: true,
        opacity: 0.7,
      });

    const missionRing = new THREE.Mesh(
      missionRingGeometry,
      missionRingMaterial
    );

    missionRing.rotation.x = Math.PI / 2;
    missionRing.position.y = -0.4;
    scene.add(missionRing);

    // Ground grid
    const grid = new THREE.GridHelper(
      22,
      22,
      0x173055,
      0x0b1627
    );

    grid.position.y = -3.5;
    scene.add(grid);

    // Ambient particles
    const particleCount = 350;
    const particlePositions = new Float32Array(
      particleCount * 3
    );

    for (let i = 0; i < particleCount; i++) {
      particlePositions[i * 3] =
        (Math.random() - 0.5) * 22;

      particlePositions[i * 3 + 1] =
        (Math.random() - 0.5) * 14;

      particlePositions[i * 3 + 2] =
        (Math.random() - 0.5) * 12;
    }

    const particleGeometry =
      new THREE.BufferGeometry();

    particleGeometry.setAttribute(
      "position",
      new THREE.BufferAttribute(
        particlePositions,
        3
      )
    );

    const particleMaterial =
      new THREE.PointsMaterial({
        color: 0x6d9fe8,
        size: 0.025,
        transparent: true,
        opacity: 0.6,
      });

    const particles = new THREE.Points(
      particleGeometry,
      particleMaterial
    );

    scene.add(particles);

    let animationFrameId: number;

    const animate = () => {
      // Core
      core.rotation.x += 0.002;
      core.rotation.y += 0.004;

      innerCore.rotation.x -= 0.003;
      innerCore.rotation.y -= 0.006;

      ringA.rotation.z += 0.003;
      ringB.rotation.z -= 0.002;

      // Nodes
      nodeGroups.forEach((group, index) => {
        const mesh = group.children[0];

        mesh.rotation.x += 0.003;
        mesh.rotation.y += 0.005;

        const pulse =
          1 +
          Math.sin(
            performance.now() * 0.002 +
              index
          ) *
            0.08;

        group.scale.setScalar(pulse);
      });

      // Mission ring
      missionRing.rotation.z += 0.0008;

      // Particles
      particles.rotation.y += 0.00015;

      renderer.render(scene, camera);

      animationFrameId =
        requestAnimationFrame(animate);
    };

    animate();

    const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();

const handleClick = (event: MouseEvent) => {
  const rect = renderer.domElement.getBoundingClientRect();

  mouse.x =
    ((event.clientX - rect.left) / rect.width) * 2 - 1;

  mouse.y =
    -((event.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);

  const intersections = raycaster.intersectObjects(
    nodeGroups,
    true
  );

  if (intersections.length === 0) {
    setSelectedNode(null);
    return;
  }

  const clickedObject = intersections[0].object;

  const nodeIndex = nodeGroups.findIndex(
    (group) =>
      group === clickedObject ||
      group.children.includes(clickedObject)
  );

  if (nodeIndex !== -1) {
    setSelectedNode(nodes[nodeIndex].name);
  }
};

    const handleResize = () => {
      camera.aspect =
        window.innerWidth /
        window.innerHeight;

      camera.updateProjectionMatrix();

      renderer.setSize(
        window.innerWidth,
        window.innerHeight
      );
    };

    window.addEventListener(
      "resize",
      handleResize
    );
    renderer.domElement.addEventListener(
  "click",
  handleClick
);

    return () => {
      cancelAnimationFrame(
        animationFrameId
      );

      window.removeEventListener(
        "resize",
        handleResize
      );
      renderer.domElement.removeEventListener(
  "click",
  handleClick
);

      coreGeometry.dispose();
      coreMaterial.dispose();
      innerGeometry.dispose();
      innerMaterial.dispose();
      ringGeometry.dispose();
      ringMaterial.dispose();
      missionRingGeometry.dispose();
      missionRingMaterial.dispose();
      particleGeometry.dispose();
      particleMaterial.dispose();
      renderer.dispose();

      if (
        mount.contains(
          renderer.domElement
        )
      ) {
        mount.removeChild(
          renderer.domElement
        );
      }
    };
  }, []);

 return (
  <div
    style={{
      width: "100vw",
      height: "100vh",
      overflow: "hidden",
      position: "relative",
    }}
  >
    <div
      ref={mountRef}
      style={{
        width: "100%",
        height: "100%",
      }}
    />

    {selectedNode && (
      <div
        style={{
          position: "absolute",
          top: "24px",
          left: "24px",
          padding: "18px 22px",
          background: "rgba(5, 7, 13, 0.9)",
          border: "1px solid rgba(80, 140, 255, 0.5)",
          borderRadius: "10px",
          color: "white",
          fontFamily: "sans-serif",
          minWidth: "180px",
        }}
      >
        <div
          style={{
            fontSize: "12px",
            opacity: 0.6,
            marginBottom: "6px",
          }}
        >
          AETHYRON NODE
        </div>

        <div
          style={{
            fontSize: "20px",
            fontWeight: 600,
          }}
        >
          {selectedNode}

{agents.length > 0 && (
  <div
    style={{
      marginTop: "8px",
      fontSize: "13px",
      opacity: 0.75,
      lineHeight: 1.4,
    }}
  >
    {
      agents.find(
        (agent) => agent.name === selectedNode
      )?.role
    }
  </div>
)}
        </div>
      </div>
    )}
  </div>
);
}

export default App;

