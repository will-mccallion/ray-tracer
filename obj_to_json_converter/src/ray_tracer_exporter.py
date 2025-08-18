# ray_tracer_exporter.py

bl_info = {
    "name": "Ray Tracer Scene Exporter (.json)",
    "author": "Your Name",
    "version": (1, 5), # Version bump for correct color export
    "blender": (4, 0, 0),
    "location": "File > Export > Ray Tracer Scene (.json)",
    "description": "Exports a Y-Up scene with correct material colors",
    "category": "Import-Export",
}

import bpy
import json
import mathutils

# --- The matrix transform logic from before is correct and unchanged ---
transform_matrix = mathutils.Matrix((
    (1, 0, 0, 0),
    (0, 0, 1, 0),
    (0, -1, 0, 0),
    (0, 0, 0, 1)
))

def export_scene(context, filepath):
    print("Starting scene export (with correct material color lookup)...")

    scene_data = {}

    # --- Camera, Lighting, and basic object loops are unchanged ---
    # ... (code for camera and lights is the same as the last version) ...
    cam_obj = context.scene.camera
    if not cam_obj: raise Exception("No active camera found.")
    cam_matrix_transformed = transform_matrix @ cam_obj.matrix_world
    look_from = cam_matrix_transformed.to_translation()
    look_dir = cam_matrix_transformed.to_quaternion() @ mathutils.Vector((0.0, 0.0, -1.0))
    look_at = look_from + look_dir
    scene_data['camera'] = { "width": context.scene.render.resolution_x, "height": context.scene.render.resolution_y, "lookfrom": {"x": look_from.x, "y": look_from.y, "z": look_from.z}, "lookat": {"x": look_at.x, "y": look_at.y, "z": look_at.z}, "vup": {"x": 0, "y": 1, "z": 0}, "vfov": bpy.data.cameras[cam_obj.data.name].angle_y * (180.0 / 3.14159265)}
    world = context.scene.world
    if world: bg_color = world.color; scene_data['background_color'] = [int(c * 255) for c in bg_color[:3]]; scene_data['ambient_light'] = {"x": bg_color[0] * 0.1, "y": bg_color[1] * 0.1, "z": bg_color[2] * 0.1}
    else: scene_data['background_color'] = [10, 10, 20]; scene_data['ambient_light'] = {"x": 0.1, "y": 0.1, "z": 0.1}
    scene_data['lights'] = []
    for light_obj in [obj for obj in context.scene.objects if obj.type == 'LIGHT']:
        if light_obj.data.type == 'POINT': light_pos = transform_matrix @ light_obj.location; intensity = light_obj.data.energy / 100.0; scene_data['lights'].append({"position": {"x": light_pos.x, "y": light_pos.y, "z": light_pos.z}, "intensity": intensity})

    scene_data['objects'] = []
    depsgraph = context.evaluated_depsgraph_get()

    for obj in [o for o in context.scene.objects if o.type == 'MESH']:
        # ... (mesh data processing is unchanged up to the color part) ...
        object_eval = obj.evaluated_get(depsgraph)
        mesh = object_eval.to_mesh()
        mesh.calc_loop_triangles()
        mesh.split_faces()
        final_matrix = transform_matrix @ obj.matrix_world
        normal_matrix = final_matrix.inverted_safe().transposed().to_quaternion()
        vertices = []
        normals = []
        for v in mesh.vertices:
            world_v = final_matrix @ v.co
            vertices.append({"x": world_v.x, "y": world_v.y, "z": world_v.z})
            world_n = normal_matrix @ v.normal
            normals.append({"x": world_n.x, "y": world_n.y, "z": world_n.z})
        indices = []
        for tri in mesh.loop_triangles:
            indices.append(list(tri.vertices))

        color = [128, 128, 128] # Default gray
        if obj.material_slots and obj.material_slots[0].material:
            mat = obj.material_slots[0].material
            # Check if the material uses nodes (it should in modern Blender)
            if mat.use_nodes and "Principled BSDF" in mat.node_tree.nodes:
                # Find the main shader node and get its "Base Color" input value
                principled_node = mat.node_tree.nodes["Principled BSDF"]
                base_color_socket = principled_node.inputs["Base Color"]
                mat_color = base_color_socket.default_value

                # Convert from (0-1 float RGBA) to (0-255 integer RGB)
                color = [int(c * 255) for c in mat_color[:3]]

        object_data = {
            "type": "Mesh",
            "vertices": vertices,
            "indices": indices,
            "normals": normals,
            "material": { "type": "Lambertian", "color": color }
        }
        scene_data['objects'].append(object_data)

        object_eval.to_mesh_clear()

    # --- WRITE THE JSON FILE ---
    with open(filepath, 'w') as f:
        json.dump(scene_data, f, indent=2)

    print(f"Scene successfully exported to {filepath}")
    return {'FINISHED'}

# --- Boilerplate is unchanged ---
from bpy_extras.io_utils import ExportHelper
from bpy.props import StringProperty
from bpy.types import Operator

class ExportMyRayTracerScene(Operator, ExportHelper):
    bl_idname = "export_scene.my_ray_tracer"
    bl_label = "Export Ray Tracer Scene"
    filename_ext = ".json"
    filter_glob: StringProperty(default="*.json", options={'HIDDEN'}, maxlen=255)
    def execute(self, context): return export_scene(context, self.filepath)
def menu_func_export(self, context): self.layout.operator(ExportMyRayTracerScene.bl_idname, text="Ray Tracer Scene (.json)")
def register(): bpy.utils.register_class(ExportMyRayTracerScene); bpy.types.TOPBAR_MT_file_export.append(menu_func_export)
def unregister(): bpy.utils.unregister_class(ExportMyRayTracerScene); bpy.types.TOPBAR_MT_file_export.remove(menu_func_export)
if __name__ == "__main__": register()
